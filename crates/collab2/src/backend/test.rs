//! TODO: docs.

#![allow(missing_docs)]

use core::convert::Infallible;
use std::borrow::Cow;

use collab_server::SessionId;
use collab_server::message::Message;
use eerie::PeerId;
use nvimx2::backend::{ApiValue, Backend, Buffer, BufferId};
use nvimx2::notify::MaybeResult;
use nvimx2::{AsyncCtx, ByteOffset, fs};
use serde::{Deserialize, Serialize};

use crate::backend::{
    ActionForSelectedSession,
    CollabBackend,
    CollabBuffer,
    CollabFs,
    StartArgs,
    StartInfos,
    default_read_replica,
    default_search_project_root,
};

#[allow(clippy::type_complexity)]
pub struct CollabTestBackend<B> {
    inner: B,
    confirm_start_with: Option<Box<dyn FnMut(&fs::AbsPath) -> bool>>,
    clipboard: Option<SessionId>,
}

pub struct CollabTestBuffer<'a, B: Backend> {
    inner: <B as Backend>::Buffer<'a>,
    lsp_root_with: Option<
        Box<
            dyn for<'b> FnMut(
                <B as Backend>::Buffer<'b>,
            ) -> Option<fs::AbsPathBuf>,
        >,
    >,
}

pub struct CollabTestFs<B: Backend> {
    inner: <B as Backend>::Fs,
    home_dir_with: Option<
        Box<dyn FnMut(&mut <B as Backend>::Fs) -> fs::AbsPathBuf + Send>,
    >,
}

impl<B> CollabTestBackend<B> {
    pub fn confirm_start_with(
        mut self,
        fun: impl FnMut(&fs::AbsPath) -> bool + 'static,
    ) -> Self {
        self.confirm_start_with = Some(Box::new(fun) as _);
        self
    }

    pub fn new(inner: B) -> Self {
        Self { inner, clipboard: None, confirm_start_with: None }
    }
}

impl<B: Backend> CollabBackend for CollabTestBackend<B> {
    type CopySessionIdError = Infallible;
    type ReadReplicaError = Infallible;
    type SearchProjectRootError = Infallible;
    type ServerTx = futures_util::sink::Drain<Message>;
    type ServerRx = futures_util::stream::Pending<Result<Message, Infallible>>;
    type ServerTxError = Infallible;
    type ServerRxError = Infallible;
    type StartSessionError = Infallible;
    type BufferLspRootError = Infallible;

    async fn confirm_start(
        project_root: &fs::AbsPath,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> bool {
        ctx.with_backend(|this| match &mut this.confirm_start_with {
            Some(fun) => fun(project_root),
            None => true,
        })
    }

    async fn copy_session_id(
        session_id: SessionId,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<(), Self::CopySessionIdError> {
        ctx.with_backend(|this| this.clipboard = Some(session_id));
        Ok(())
    }

    async fn read_replica(
        peer_id: PeerId,
        project_root: &fs::AbsPath,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<eerie::Replica, Self::ReadReplicaError> {
        let _ = default_read_replica::read_replica(
            peer_id,
            project_root.to_owned(),
            ctx,
        )
        .await;
        todo!();
    }

    async fn search_project_root(
        buffer_id: BufferId<Self>,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<eerie::fs::AbsPathBuf, Self::SearchProjectRootError> {
        let _ = default_search_project_root::search(buffer_id, ctx).await;
        todo!()
    }

    async fn select_session<'pairs>(
        _sessions: &'pairs [(fs::AbsPathBuf, SessionId)],
        _action: ActionForSelectedSession,
        _ctx: &mut AsyncCtx<'_, Self>,
    ) -> Option<&'pairs (fs::AbsPathBuf, SessionId)> {
        todo!()
    }

    async fn start_session(
        _args: StartArgs<'_>,
        _ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<StartInfos<Self>, Self::StartSessionError> {
        todo!()
    }
}

impl<B: Backend> CollabBuffer for CollabTestBuffer<'_, B> {
    type LspRootError = Infallible;

    fn lsp_root(
        _id: Self::Id,
    ) -> Result<Option<fs::AbsPathBuf>, Self::LspRootError> {
        todo!()
    }
}

impl<B: Backend> CollabFs for CollabTestFs<B> {
    type HomeDirError = Infallible;

    async fn home_dir(
        &mut self,
    ) -> Result<fs::AbsPathBuf, Self::HomeDirError> {
        todo!()
    }
}

impl<B: Backend> Backend for CollabTestBackend<B> {
    type Api = <B as Backend>::Api;
    type Buffer<'a> = CollabTestBuffer<'a, B>;
    type BufferId = <B as Backend>::BufferId;
    type LocalExecutor = <B as Backend>::LocalExecutor;
    type BackgroundExecutor = <B as Backend>::BackgroundExecutor;
    type Fs = CollabTestFs<B>;
    type Emitter<'this> = <B as Backend>::Emitter<'this>;
    type SerializeError = <B as Backend>::SerializeError;
    type DeserializeError = <B as Backend>::DeserializeError;

    fn buffer(&mut self, _id: BufferId<Self>) -> Option<Self::Buffer<'_>> {
        todo!();
    }

    fn buffer_ids(&mut self) -> impl Iterator<Item = BufferId<Self>> + use<B> {
        self.inner.buffer_ids()
    }

    fn current_buffer(&mut self) -> Option<Self::Buffer<'_>> {
        todo!();
    }

    fn fs(&mut self) -> Self::Fs {
        todo!();
    }

    fn emitter(&mut self) -> Self::Emitter<'_> {
        self.inner.emitter()
    }

    fn local_executor(&mut self) -> &mut Self::LocalExecutor {
        self.inner.local_executor()
    }

    fn background_executor(&mut self) -> &mut Self::BackgroundExecutor {
        self.inner.background_executor()
    }

    fn serialize<V>(
        &mut self,
        value: &V,
    ) -> impl MaybeResult<ApiValue<Self>, Error = Self::SerializeError>
    where
        V: ?Sized + Serialize,
    {
        self.inner.serialize(value)
    }

    fn deserialize<'de, V>(
        &mut self,
        value: ApiValue<Self>,
    ) -> impl MaybeResult<V, Error = Self::DeserializeError>
    where
        V: Deserialize<'de>,
    {
        self.inner.deserialize(value)
    }
}

impl<B: Backend> Buffer for CollabTestBuffer<'_, B> {
    type Id = <B as Backend>::BufferId;

    fn byte_len(&self) -> ByteOffset {
        self.inner.byte_len()
    }

    fn id(&self) -> Self::Id {
        self.inner.id()
    }

    fn name(&self) -> Cow<'_, str> {
        self.inner.name()
    }
}

impl<B: Backend> fs::Fs for CollabTestFs<B> {
    type Timestamp = <B::Fs as fs::Fs>::Timestamp;
    type DirEntry = <B::Fs as fs::Fs>::DirEntry;
    type Directory = <B::Fs as fs::Fs>::Directory;
    type File = <B::Fs as fs::Fs>::File;
    type Symlink = fs::SameSymlink<Self, B::Fs>;
    type ReadDir = <B::Fs as fs::Fs>::ReadDir;
    type ReadDirEntryError = <B::Fs as fs::Fs>::ReadDirEntryError;
    type NodeAtPathError = <B::Fs as fs::Fs>::NodeAtPathError;
    type ReadDirError = <B::Fs as fs::Fs>::ReadDirError;
    type Watcher = <B::Fs as fs::Fs>::Watcher;
    type WatchError = <B::Fs as fs::Fs>::WatchError;

    async fn node_at_path<P: AsRef<fs::AbsPath>>(
        &self,
        _path: P,
    ) -> Result<Option<fs::FsNode<Self>>, Self::NodeAtPathError> {
        todo!();
    }

    fn now(&self) -> Self::Timestamp {
        self.inner.now()
    }

    async fn read_dir<P: AsRef<fs::AbsPath>>(
        &self,
        dir_path: P,
    ) -> Result<Self::ReadDir, Self::ReadDirError> {
        self.inner.read_dir(dir_path).await
    }

    async fn watch<P: AsRef<fs::AbsPath>>(
        &self,
        path: P,
    ) -> Result<Self::Watcher, Self::WatchError> {
        self.inner.watch(path).await
    }
}

impl<B: Default> Default for CollabTestBackend<B> {
    fn default() -> Self {
        Self::new(B::default())
    }
}
