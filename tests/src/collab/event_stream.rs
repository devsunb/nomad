use abs_path::{AbsPath, path};
use collab::event::{BufferEvent, Event, EventStream};
use collab::mock::CollabMock;
use collab::{CollabEditor, PeerId};
use editor::{Buffer, Context, Replacement};
use mock::{EditorExt, Mock};

#[test]
fn editing_watched_buffer_emits_event() {
    let fs = mock::fs! {
        "foo.txt": "hello world",
    };

    CollabMock::new(Mock::new(fs)).block_on(async |ctx| {
        let agent_id = ctx.new_agent_id();

        let foo_id =
            ctx.create_buffer(path!("/foo.txt"), agent_id).await.unwrap();

        let mut event_stream = EventStream::new(path!("/"), ctx).await;

        ctx.with_borrowed(|ctx| {
            let _ = ctx
                .buffer(foo_id)
                .unwrap()
                .schedule_insertion(5, ", ", agent_id);
        });

        let (buffer_id, replacements) = event_stream.next_as_edit(ctx).await;
        assert_eq!(buffer_id, foo_id);
        assert_eq!(replacements.as_ref(), [Replacement::insertion(5, ", ")]);
    });
}

trait EventStreamExt<Ed: CollabEditor> {
    fn event_stream(&mut self) -> &mut EventStream<Ed>;

    /// Creates a new [`EventStream`] for the project rooted at the given
    /// path.
    fn new(
        project_root_path: &AbsPath,
        ctx: &mut Context<Ed>,
    ) -> impl Future<Output = EventStream<Ed>> {
        async {
            let (_, event_stream, _) =
                collab::start::Start::<Ed>::read_project(
                    project_root_path,
                    PeerId::new(1),
                    ctx,
                )
                .await
                .unwrap();

            event_stream
        }
    }

    fn next_as_buffer(
        &mut self,
        ctx: &mut Context<Ed>,
    ) -> impl Future<Output = BufferEvent<Ed>> {
        async move {
            match self.event_stream().next(ctx).await {
                Ok(Event::Buffer(event)) => event,
                Ok(other) => panic!("expected BufferEvent, got {other:?}"),
                Err(err) => panic!("{err}"),
            }
        }
    }

    fn next_as_edit(
        &mut self,
        ctx: &mut Context<Ed>,
    ) -> impl Future<Output = (Ed::BufferId, impl AsRef<[Replacement]>)> {
        async move {
            match self.next_as_buffer(ctx).await {
                BufferEvent::Edited(buffer_id, replacements) => {
                    (buffer_id, replacements)
                },
                other => panic!("expected Edited event, got {other:?}"),
            }
        }
    }
}

impl<Ed: CollabEditor> EventStreamExt<Ed> for EventStream<Ed> {
    fn event_stream(&mut self) -> &mut EventStream<Ed> {
        self
    }
}
