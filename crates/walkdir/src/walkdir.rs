use core::convert::Infallible;
use core::error::Error;
use core::fmt;
use core::pin::Pin;

use ed::fs::{self, AbsPath, AbsPathBuf, Directory, Metadata};
use futures_util::stream::{self, FusedStream, Stream, StreamExt};
use futures_util::{FutureExt, pin_mut, select};

use crate::dir_entry::DirEntry;
use crate::filter::{Either, Filter, Filter2, Filtered, Filtered2};

/// TODO: docs.
pub type DirEntri<Fs> = <<Fs as fs::Fs>::Directory as fs::Directory>::Metadata;

/// TODO: docs.
pub trait WalkDir2<Fs: fs::Fs>: Sized {
    /// The type of error that can occur when reading a directory fails.
    type ReadError: Error;

    /// The type of error that can occur when reading a specific entry in a
    /// directory fails.
    type ReadEntryError: Error;

    /// TODO: docs.
    fn read_dir(
        &self,
        dir_path: &AbsPath,
    ) -> impl Future<
        Output = Result<
            impl FusedStream<Item = Result<DirEntri<Fs>, Self::ReadEntryError>>,
            Self::ReadError,
        >,
    >;

    /// TODO: docs.
    #[inline]
    fn filter<F>(self, filter: F) -> Filtered2<F, Self>
    where
        F: Filter2<Fs>,
    {
        Filtered2::new(filter, self)
    }

    /// TODO: docs.
    #[allow(clippy::type_complexity)]
    #[inline]
    fn for_each<'a, H, E>(
        &'a self,
        dir_path: &'a AbsPath,
        handler: H,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), WalkDirError2<Fs, Self, E>>> + 'a>,
    >
    where
        H: AsyncFn(&AbsPath, DirEntri<Fs>) -> Result<(), E> + Clone + 'a,
        E: 'a,
    {
        Box::pin(async move {
            let entries = self
                .read_dir(dir_path)
                .await
                .map_err(WalkDirError2::ReadDir)?;
            let mut handle_entries = stream::FuturesUnordered::new();
            let mut read_children = stream::FuturesUnordered::new();
            pin_mut!(entries);
            loop {
                select! {
                    res = entries.select_next_some() => {
                        let entry = res.map_err(WalkDirError2::ReadEntry)?;
                        let node_kind = entry
                            .node_kind()
                            .await
                            .map_err(WalkDirError2::NodeKind)?;
                        if node_kind.is_dir() {
                            let dir_name = entry
                                .name()
                                .await
                                .map_err(WalkDirError2::NodeName)?;
                            let dir_path = dir_path.join(&dir_name);
                            let handler = handler.clone();
                            read_children.push(async move {
                                self.for_each(&dir_path, handler).await
                            });
                        }
                        let handler = &handler;
                        handle_entries.push(async move {
                            handler(dir_path, entry).await
                        });
                    },
                    res = read_children.select_next_some() => res?,
                    res = handle_entries.select_next_some() => {
                        res.map_err(WalkDirError2::Other)?;
                    },
                    complete => return Ok(()),
                }
            }
        })
    }

    /// TODO: docs.
    #[inline]
    fn paths<'a>(
        &'a self,
        dir_path: &'a AbsPath,
    ) -> impl FusedStream<
        Item = Result<
            AbsPathBuf,
            WalkDirError2<Fs, Self, <DirEntri<Fs> as Metadata>::NameError>,
        >,
    > + 'a {
        self.to_stream(dir_path, async |dir_path, entry| {
            entry.name().await.map(|name| dir_path.join(&name))
        })
    }

    /// TODO: docs.
    #[inline]
    fn to_stream<'a, H, T, E>(
        &'a self,
        dir_path: &'a AbsPath,
        handler: H,
    ) -> impl FusedStream<Item = Result<T, WalkDirError2<Fs, Self, E>>> + 'a
    where
        H: AsyncFn(&AbsPath, DirEntri<Fs>) -> Result<T, E> + Clone + 'a,
        T: 'a,
        E: 'a,
    {
        let (tx, rx) = flume::unbounded();
        let for_each = self
            .for_each(dir_path, async move |dir_path, entry| {
                let _ = tx.send(handler(dir_path, entry).await?);
                Ok(())
            })
            .boxed_local()
            .fuse();
        futures_util::stream::unfold(
            (for_each, rx),
            move |(mut for_each, rx)| async move {
                let res = select! {
                    res = for_each => match res {
                        Ok(()) => return None,
                        Err(err) => Err(err),
                    },
                    res = rx.recv_async() => match res {
                        Ok(value) => Ok(value),
                        Err(_err) => return None,
                    },
                };
                Some((res, (for_each, rx)))
            },
        )
    }
}

/// TODO: docs.
pub enum WalkDirError2<Fs, W, T>
where
    Fs: fs::Fs,
    W: WalkDir2<Fs>,
{
    /// TODO: docs.
    Other(T),

    /// TODO: docs.
    NodeKind(<DirEntri<Fs> as Metadata>::NodeKindError),

    /// TODO: docs.
    NodeName(<DirEntri<Fs> as Metadata>::NameError),

    /// TODO: docs.
    ReadDir(W::ReadError),

    /// TODO: docs.
    ReadEntry(W::ReadEntryError),
}

/// TODO: docs.
pub trait WalkDir: Sized {
    /// TODO: docs.
    type Fs: fs::Fs;

    /// TODO: docs.
    type DirEntry: fs::Metadata<Timestamp = <Self::Fs as fs::Fs>::Timestamp>;

    /// TODO: docs.
    type ReadDirError: Error;

    /// TODO: docs.
    type ReadDirEntryError: Error;

    /// TODO: docs.
    fn read_dir(
        &self,
        dir_path: &fs::AbsPath,
    ) -> impl Future<
        Output = Result<
            impl Stream<Item = Result<Self::DirEntry, Self::ReadDirEntryError>>,
            Self::ReadDirError,
        >,
    >;

    /// TODO: docs.
    #[inline]
    fn filter<F>(self, filter: F) -> Filtered<F, Self>
    where
        F: Filter<Self>,
    {
        Filtered::new(filter, self)
    }

    /// TODO: docs.
    #[allow(clippy::type_complexity)]
    #[inline]
    fn for_each<'a, H, E>(
        &'a self,
        dir_path: &'a fs::AbsPath,
        handler: H,
    ) -> Pin<Box<dyn Future<Output = Result<(), ForEachError<Self, E>>> + 'a>>
    where
        H: AsyncFn(DirEntry<Self>) -> Result<(), E> + Clone + 'a,
        E: 'a,
    {
        Box::pin(async move {
            let entries = match self.read_dir(dir_path).await {
                Ok(entries) => entries.fuse(),
                Err(err) => {
                    return Err(ForEachError {
                        dir_path: dir_path.to_owned(),
                        kind: Either::Left(WalkErrorKind::ReadDir(err)),
                    });
                },
            };
            let mut create_entries = stream::FuturesUnordered::new();
            let mut handle_entries = stream::FuturesUnordered::new();
            let mut read_children = stream::FuturesUnordered::new();
            pin_mut!(entries);
            loop {
                select! {
                    res = entries.select_next_some() => {
                        let entry = res.map_err(|err| ForEachError {
                            dir_path: dir_path.to_owned(),
                            kind: Either::Left(WalkErrorKind::DirEntry(err)),
                        })?;
                        create_entries.push(DirEntry::new(dir_path, entry));
                    },
                    res = create_entries.select_next_some() => {
                        let entry = res.map_err(|kind| ForEachError {
                            dir_path: dir_path.to_owned(),
                            kind: Either::Left(kind),
                        })?;
                        if entry.node_kind().is_dir() {
                            let dir_path = entry.path();
                            let handler = handler.clone();
                            read_children.push(async move {
                                self.for_each(&dir_path, handler).await
                            });
                        }
                        let handler = &handler;
                        handle_entries.push(async move {
                            let parent_path = entry.parent_path();
                            handler(entry).await.map_err(|err| {
                                ForEachError {
                                    dir_path: parent_path.to_owned(),
                                    kind: Either::Right(err),
                                }
                            })
                        });
                    },
                    res = read_children.select_next_some() => res?,
                    res = handle_entries.select_next_some() => res?,
                    complete => return Ok(()),
                }
            }
        })
    }

    /// TODO: docs.
    #[inline]
    fn paths<'a>(
        &'a self,
        dir_path: &'a fs::AbsPath,
    ) -> impl Stream<Item = Result<fs::AbsPathBuf, PathsError<Self>>> + 'a
    {
        self.to_stream(dir_path, async |entry| {
            Ok::<_, Infallible>(entry.path())
        })
        .map(|res| {
            res.map_err(|err| {
                err.map_kind(|kind| match kind {
                    Either::Left(res) => res,
                    Either::Right(_infallible) => unreachable!(),
                })
            })
        })
    }

    /// TODO: docs.
    #[inline]
    fn to_stream<'a, H, T, E>(
        &'a self,
        dir_path: &'a fs::AbsPath,
        handler: H,
    ) -> impl Stream<Item = Result<T, ForEachError<Self, E>>> + 'a
    where
        H: AsyncFn(DirEntry<Self>) -> Result<T, E> + Clone + 'a,
        T: 'a,
        E: 'a,
    {
        let (tx, rx) = flume::unbounded();
        let for_each = self
            .for_each(dir_path, async move |entry| {
                let _ = tx.send(handler(entry).await?);
                Ok(())
            })
            .boxed_local()
            .fuse();
        futures_util::stream::unfold(
            (for_each, rx),
            move |(mut for_each, rx)| async move {
                let res = select! {
                    res = for_each => match res {
                        Ok(()) => return None,
                        Err(err) => Err(err),
                    },
                    res = rx.recv_async() => match res {
                        Ok(value) => Ok(value),
                        Err(_err) => return None,
                    },
                };
                Some((res, (for_each, rx)))
            },
        )
    }
}

/// TODO: docs.
pub type ForEachError<W, E> = WalkError<Either<WalkErrorKind<W>, E>>;

/// TODO: docs.
pub type PathsError<W> = WalkError<WalkErrorKind<W>>;

/// TODO: docs.
#[derive(Debug, PartialEq)]
pub struct WalkError<K> {
    /// TODO: docs.
    pub dir_path: fs::AbsPathBuf,

    /// TODO: docs.
    pub kind: K,
}

/// TODO: docs.
#[derive(derive_more::Debug)]
#[debug(bound(W: WalkDir))]
pub enum WalkErrorKind<W: WalkDir> {
    /// TODO: docs.
    DirEntry(W::ReadDirEntryError),

    /// TODO: docs.
    DirEntryName(<W::DirEntry as fs::Metadata>::NameError),

    /// TODO: docs.
    DirEntryNodeKind(<W::DirEntry as fs::Metadata>::NodeKindError),

    /// TODO: docs.
    ReadDir(W::ReadDirError),
}

impl<K> WalkError<K> {
    /// TODO: docs.
    pub fn map_kind<F, K2>(self, f: F) -> WalkError<K2>
    where
        F: FnOnce(K) -> K2,
    {
        WalkError { dir_path: self.dir_path, kind: f(self.kind) }
    }
}

/// TODO: docs.
#[derive(derive_more::Debug)]
#[debug(bound(Fs: fs::Fs))]
pub enum FsReadDirError<Fs: fs::Fs> {
    /// TODO: docs.
    NoNodeAtPath,

    /// TODO: docs.
    NodeAtPath(Fs::NodeAtPathError),

    /// TODO: docs.
    ReadDir(<Fs::Directory as fs::Directory>::ReadError),

    /// TODO: docs.
    ReadFile,

    /// TODO: docs.
    ReadSymlink,
}

impl<Fs: fs::Fs> WalkDir for Fs {
    type Fs = Self;

    type DirEntry = <<Self as fs::Fs>::Directory as fs::Directory>::Metadata;

    type ReadDirEntryError =
        <<Self as fs::Fs>::Directory as fs::Directory>::ReadEntryError;

    type ReadDirError = FsReadDirError<Self>;

    async fn read_dir(
        &self,
        dir_path: &fs::AbsPath,
    ) -> Result<
        impl Stream<Item = Result<Self::DirEntry, Self::ReadDirEntryError>>,
        Self::ReadDirError,
    > {
        let Some(node) = self
            .node_at_path(dir_path)
            .await
            .map_err(FsReadDirError::NodeAtPath)?
        else {
            return Err(FsReadDirError::NoNodeAtPath);
        };

        match node {
            fs::FsNode::Directory(dir) => {
                dir.read().await.map_err(FsReadDirError::ReadDir)
            },
            fs::FsNode::File(_) => Err(FsReadDirError::ReadFile),
            fs::FsNode::Symlink(_) => Err(FsReadDirError::ReadSymlink),
        }
    }
}

impl<W: fmt::Display> fmt::Display for WalkError<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error at {:?}: {}", self.dir_path, self.kind)
    }
}

impl<W: WalkDir> fmt::Display for WalkErrorKind<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalkErrorKind::DirEntry(err) => err.fmt(f),
            WalkErrorKind::DirEntryName(err) => err.fmt(f),
            WalkErrorKind::DirEntryNodeKind(err) => err.fmt(f),
            WalkErrorKind::ReadDir(err) => err.fmt(f),
        }
    }
}

impl<W> PartialEq for WalkErrorKind<W>
where
    W: WalkDir,
    W::ReadDirError: PartialEq,
    W::ReadDirEntryError: PartialEq,
    <W::DirEntry as fs::Metadata>::NameError: PartialEq,
    <W::DirEntry as fs::Metadata>::NodeKindError: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        use WalkErrorKind::*;

        match (self, other) {
            (DirEntry(l), DirEntry(r)) => l == r,
            (DirEntryName(l), DirEntryName(r)) => l == r,
            (DirEntryNodeKind(l), DirEntryNodeKind(r)) => l == r,
            (ReadDir(l), ReadDir(r)) => l == r,
            _ => false,
        }
    }
}

impl<Fs: fs::Fs> PartialEq for FsReadDirError<Fs>
where
    Fs::NodeAtPathError: PartialEq,
    <Fs::Directory as fs::Directory>::ReadError: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        use FsReadDirError::*;

        match (self, other) {
            (NoNodeAtPath, NoNodeAtPath) => true,
            (NodeAtPath(l), NodeAtPath(r)) => l == r,
            (ReadDir(l), ReadDir(r)) => l == r,
            (ReadFile, ReadFile) => true,
            (ReadSymlink, ReadSymlink) => true,
            _ => false,
        }
    }
}

impl<Fs: fs::Fs> fmt::Display for FsReadDirError<Fs> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsReadDirError::NoNodeAtPath => {
                write!(f, "no node at path")
            },
            FsReadDirError::NodeAtPath(err) => {
                write!(f, "couldn't get file or directory: {err}")
            },
            FsReadDirError::ReadDir(err) => {
                write!(f, "couldn't read directory: {err}")
            },
            FsReadDirError::ReadFile => {
                write!(f, "couldn't read file at path")
            },
            FsReadDirError::ReadSymlink => {
                write!(f, "couldn't read symlink at path")
            },
        }
    }
}

impl<Fs: fs::Fs> Error for FsReadDirError<Fs> {}
