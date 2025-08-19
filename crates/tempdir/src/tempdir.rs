use abs_path::{AbsPath, NodeName};
use fs::{Directory, Fs};
use futures_util::Stream;

/// TODO: docs.
pub struct TempDir {
    /// We need to keep the inner `TempDir` around so that the directory can
    /// be deleted when it is dropped.
    _inner: tempdir_inner::TempDir,
    os_dir: real_fs::Directory,
}

impl TempDir {
    pub(crate) fn new(
        inner: tempdir_inner::TempDir,
        os_dir: real_fs::Directory,
    ) -> Self {
        Self { _inner: inner, os_dir }
    }
}

impl Directory for TempDir {
    type EventStream = <real_fs::Directory as Directory>::EventStream;
    type Fs = <real_fs::Directory as Directory>::Fs;

    type ClearError = <real_fs::Directory as Directory>::ClearError;
    type CreateDirectoryError =
        <real_fs::Directory as Directory>::CreateDirectoryError;
    type CreateFileError = <real_fs::Directory as Directory>::CreateFileError;
    type CreateSymlinkError =
        <real_fs::Directory as Directory>::CreateSymlinkError;
    type DeleteError = <real_fs::Directory as Directory>::DeleteError;
    type ListError = <real_fs::Directory as Directory>::ListError;
    type MoveError = <real_fs::Directory as Directory>::MoveError;
    type ParentError = <real_fs::Directory as Directory>::ParentError;
    type ReadMetadataError =
        <real_fs::Directory as Directory>::ReadMetadataError;

    async fn create_directory(
        &self,
        directory_name: &NodeName,
    ) -> Result<<Self::Fs as Fs>::Directory, Self::CreateDirectoryError> {
        <real_fs::Directory as Directory>::create_directory(
            &self.os_dir,
            directory_name,
        )
        .await
    }

    async fn create_file(
        &self,
        file_name: &NodeName,
    ) -> Result<<Self::Fs as Fs>::File, Self::CreateFileError> {
        <real_fs::Directory as Directory>::create_file(&self.os_dir, file_name)
            .await
    }

    async fn create_symlink(
        &self,
        symlink_name: &NodeName,
        target_path: &str,
    ) -> Result<<Self::Fs as Fs>::Symlink, Self::CreateSymlinkError> {
        <real_fs::Directory as Directory>::create_symlink(
            &self.os_dir,
            symlink_name,
            target_path,
        )
        .await
    }

    async fn clear(&self) -> Result<(), Self::ClearError> {
        <real_fs::Directory as Directory>::clear(&self.os_dir).await
    }

    async fn delete(self) -> Result<(), Self::DeleteError> {
        <real_fs::Directory as Directory>::delete(self.os_dir).await
    }

    fn meta(&self) -> <Self::Fs as Fs>::Metadata {
        <real_fs::Directory as Directory>::meta(&self.os_dir)
    }

    async fn r#move(&self, new_path: &AbsPath) -> Result<(), Self::MoveError> {
        <real_fs::Directory as Directory>::r#move(&self.os_dir, new_path).await
    }

    async fn parent(
        &self,
    ) -> Result<Option<<Self::Fs as Fs>::Directory>, Self::ParentError> {
        <real_fs::Directory as Directory>::parent(&self.os_dir).await
    }

    fn path(&self) -> &AbsPath {
        <real_fs::Directory as Directory>::path(&self.os_dir)
    }

    async fn list_metas(
        &self,
    ) -> Result<
        impl Stream<
            Item = Result<<Self::Fs as Fs>::Metadata, Self::ReadMetadataError>,
        > + Send
        + use<>,
        Self::ListError,
    > {
        <real_fs::Directory as Directory>::list_metas(&self.os_dir).await
    }

    fn watch(&self) -> Self::EventStream {
        <real_fs::Directory as Directory>::watch(&self.os_dir)
    }
}
