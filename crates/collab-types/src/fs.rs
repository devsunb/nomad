//! Contains the message types related to file system operations.

use std::sync::Arc;

pub use puff::file::GlobalFileId;
pub use puff::ops::{
    DirectoryDeletion,
    DirectoryMove,
    FileDeletion,
    FileMove,
    Rename,
};

/// The message representing a directory creation.
pub type DirectoryCreation =
    puff::ops::DirectoryCreation<NewDirectoryContents>;

/// The message representing a file creation.
pub type FileCreation = puff::ops::FileCreation<NewFileContents>;

/// The contents of a newly created directory.
pub type NewDirectoryContents = ();

/// The contents of a newly created file.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NewFileContents {
    /// The file contains arbitrary binary data.
    Binary(bytes::Bytes),

    /// The file is a symlink to the given target path.
    Symlink(Arc<str>),

    /// The file contains UTF-8 encoded text.
    Text(crop::Rope),
}

#[cfg(feature = "serde")]
mod encode {
    use puff::encode::{
        Buffer,
        ByteDecodeError,
        Decode,
        Encode,
        IntDecodeError,
    };

    use super::*;

    /// The type of error returned when decoding a [`NewFileContents`] fails.
    #[derive(Debug, derive_more::Display, cauchy::Error, cauchy::From)]
    #[display("{_0}")]
    pub enum NewFileContentsDecodeError {
        Kind(#[from] NewFileContentsKindDecodeError),
        Symlink(core::str::Utf8Error),
        Text(core::str::Utf8Error),
        Int(#[from] IntDecodeError),
        #[display(
            "not enough bytes in buffer: expected: {expected}, found {actual}"
        )]
        NotEnoughBytes {
            actual: usize,
            expected: usize,
        },
    }

    /// The type of error returned when decoding a [`NewFileContentsKind`]
    /// fails.
    #[derive(Debug, derive_more::Display, cauchy::From, cauchy::Error)]
    pub enum NewFileContentsKindDecodeError {
        #[display("{_0}")]
        EmptyBuffer(#[from] ByteDecodeError),
        #[display("invalid tag: {_0}, expected 0, 1, or 2")]
        InvalidTag(u8),
    }

    #[derive(Debug, Copy, Clone)]
    #[repr(u8)]
    enum NewFileContentsKind {
        Binary = 0,
        Symlink = 1,
        Text = 2,
    }

    impl NewFileContents {
        #[inline]
        fn kind(&self) -> NewFileContentsKind {
            match self {
                Self::Binary(_) => NewFileContentsKind::Binary,
                Self::Symlink(_) => NewFileContentsKind::Symlink,
                Self::Text(_) => NewFileContentsKind::Text,
            }
        }
    }

    impl Encode for NewFileContents {
        #[track_caller]
        #[inline]
        fn encode(&self, buf: &mut impl Buffer) {
            self.kind().encode(buf);

            match self {
                Self::Binary(bytes) => {
                    u64::try_from(bytes.len())
                        .expect("bytes length exceeds u64::MAX")
                        .encode(buf);
                    buf.extend(bytes)
                },
                NewFileContents::Symlink(target) => (&**target).encode(buf),
                NewFileContents::Text(text) => {
                    u64::try_from(text.byte_len())
                        .expect("text byte length exceeds u64::MAX")
                        .encode(buf);
                    for chunk in text.chunks() {
                        buf.extend(chunk.as_bytes());
                    }
                },
            }
        }
    }

    impl Decode for NewFileContents {
        type Error = NewFileContentsDecodeError;

        #[inline]
        fn decode(buf: &mut &[u8]) -> Result<Self, Self::Error> {
            let contents_len = usize::try_from(u64::decode(buf)?)
                .expect("buffer length exceeds usize::MAX");

            if buf.len() < contents_len {
                return Err(NewFileContentsDecodeError::NotEnoughBytes {
                    actual: buf.len(),
                    expected: contents_len,
                });
            }

            let (data, rest) = buf.split_at(contents_len);
            *buf = rest;

            match NewFileContentsKind::decode(buf)? {
                NewFileContentsKind::Binary => {
                    Ok(Self::Binary(bytes::Bytes::copy_from_slice(data)))
                },

                NewFileContentsKind::Symlink => str::from_utf8(data)
                    .map_err(NewFileContentsDecodeError::Symlink)
                    .map(Arc::from)
                    .map(Self::Symlink),

                NewFileContentsKind::Text => str::from_utf8(data)
                    .map_err(NewFileContentsDecodeError::Text)
                    .map(crop::Rope::from)
                    .map(Self::Text),
            }
        }
    }

    impl Encode for NewFileContentsKind {
        #[inline]
        fn encode(&self, buf: &mut impl Buffer) {
            (*self as u8).encode(buf)
        }
    }

    impl Decode for NewFileContentsKind {
        type Error = NewFileContentsKindDecodeError;

        #[inline]
        fn decode(buf: &mut &[u8]) -> Result<Self, Self::Error> {
            match u8::decode(buf)? {
                0 => Ok(Self::Binary),
                1 => Ok(Self::Symlink),
                2 => Ok(Self::Text),
                other => Err(Self::Error::InvalidTag(other)),
            }
        }
    }
}
