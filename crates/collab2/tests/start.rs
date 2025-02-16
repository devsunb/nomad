#![allow(missing_docs)]

use auth::Auth;
use collab2::Collab;
use collab2::backend::test::CollabTestBackend;
use collab2::start::StartError;
use nvimx2::action::AsyncAction;
use nvimx2::backend::Backend;
use nvimx2::tests::TestBackend;

#[test]
fn cannot_start_session_if_not_logged_in() {
    CollabTestBackend::<TestBackend>::default().with_async_ctx(
        async move |ctx| {
            let collab = Collab::from(&Auth::default());
            let err = collab.start().call((), ctx).await.unwrap_err();
            assert!(matches!(err, StartError::UserNotLoggedIn(_)));
        },
    );
}
