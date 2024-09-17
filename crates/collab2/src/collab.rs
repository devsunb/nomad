use core::future::Future;

use collab_server::SessionId;
use futures_util::{select, FutureExt, StreamExt};
use nomad2::{
    module_name,
    Api,
    Context,
    Editor,
    Module,
    ModuleName,
    Subsctiption,
};

use crate::actions::{JoinSession, StartSession};
use crate::{Config, Session};

/// TODO: docs.
pub struct Collab {
    config: Config,
    join_sub: Subscription<JoinSession>,
    start_sub: Subscription<StartSession>,
}

impl Collab {
    fn join_session<E: Editor>(
        &self,
        id: SessionId,
        ctx: Context<E>,
    ) -> impl Future<Output = ()> + 'static {
        let config = self.config.clone();

        async move {
            let session = match Session::join(id, config, ctx).await {
                Ok(session) => session,
                Err(err) => {
                    println!("{err:?}");
                    return;
                },
            };

            if let Err(err) = session.run().await {
                println!("{err}");
            }
        }
    }

    fn start_session<E: Editor>(
        &self,
        ctx: Context<E>,
    ) -> impl Future<Output = ()> + 'static {
        let config = self.config.clone();

        async move {
            let session = match Session::start(config, ctx).await {
                Ok(session) => session,
                Err(err) => {
                    println!("{err:?}");
                    return;
                },
            };

            if let Err(err) = session.run().await {
                println!("{err}");
            }
        }
    }
}

impl<E: Editor> Module<E> for Collab {
    const NAME: ModuleName = module_name!("collab");

    type Config = Config;

    fn init(_ctx: &Context<E>) -> Api<E, Self> {
        todo!();
    }

    async fn run(&mut self, ctx: &Context<E>) {
        loop {
            select! {
                _ = self.start_sub.next().fuse() => {
                    let fut = self.start_session(ctx.clone());
                    ctx.spawner().spawn(fut).detach();
                },
                session_id = self.join_sub.next().fuse() => {
                    let fut = self.join_session(session_id, ctx.clone());
                    ctx.spawner().spawn(fut).detach();
                },
            }
        }
    }
}
