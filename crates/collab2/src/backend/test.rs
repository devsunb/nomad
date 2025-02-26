//! TODO: docs.

#![allow(missing_docs)]

use core::convert::Infallible;

use collab_server::SessionId;
use collab_server::message::Message;
use eerie::PeerId;
use nvimx2::backend::{ApiValue, Backend, Buffer, BufferId};
use nvimx2::notify::MaybeResult;
use nvimx2::{AsyncCtx, fs};
use serde::{Deserialize, Serialize};

use crate::backend::{
    ActionForSelectedSession,
    CollabBackend,
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
    type ServerRx = futures_util::stream::Pending<Result<Message, Infallible>>;
    type ServerTx = futures_util::sink::Drain<Message>;

    type CopySessionIdError = Infallible;
    type HomeDirError = Infallible;
    type LspRootError = Infallible;
    type ReadReplicaError = Infallible;
    type SearchProjectRootError = Infallible;
    type ServerRxError = Infallible;
    type ServerTxError = Infallible;
    type StartSessionError = Infallible;

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

    async fn home_dir(
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<fs::AbsPathBuf, Self::HomeDirError> {
        todo!()
    }

    fn lsp_root(
        id: <Self::Buffer<'_> as Buffer>::Id,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> Result<Option<fs::AbsPathBuf>, Self::LspRootError> {
        todo!()
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

impl<B: Backend> Backend for CollabTestBackend<B> {
    type Api = <B as Backend>::Api;
    type Buffer<'a> = <B as Backend>::Buffer<'a>;
    type BufferId = <B as Backend>::BufferId;
    type LocalExecutor = <B as Backend>::LocalExecutor;
    type BackgroundExecutor = <B as Backend>::BackgroundExecutor;
    type Fs = <B as Backend>::Fs;
    type Emitter<'this> = <B as Backend>::Emitter<'this>;
    type SerializeError = <B as Backend>::SerializeError;
    type DeserializeError = <B as Backend>::DeserializeError;

    fn buffer(&mut self, id: BufferId<Self>) -> Option<Self::Buffer<'_>> {
        self.inner.buffer(id)
    }

    fn buffer_ids(&mut self) -> impl Iterator<Item = BufferId<Self>> + use<B> {
        self.inner.buffer_ids()
    }

    fn current_buffer(&mut self) -> Option<Self::Buffer<'_>> {
        self.inner.current_buffer()
    }

    fn fs(&mut self) -> Self::Fs {
        self.inner.fs()
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

impl<B: Default> Default for CollabTestBackend<B> {
    fn default() -> Self {
        Self::new(B::default())
    }
}
