use nomad::ctx::BufferCtx;
use nomad::{BufferId, ByteOffset};

type Peer = String;

/// TODO: docs.
pub(super) struct PeerTooltip {
    at_offset: ByteOffset,
    in_buffer: BufferId,
    peer: Peer,
}

impl PeerTooltip {
    /// The [`BufferId`] this tooltip is in.
    pub(super) fn buffer_id(&self) -> BufferId {
        self.in_buffer
    }

    pub(super) fn create(
        peer: Peer,
        at_offset: ByteOffset,
        ctx: BufferCtx<'_>,
    ) -> Self {
        Self { at_offset, in_buffer: ctx.buffer_id(), peer }
    }

    /// The [`Peer`] this tooltip is for.
    pub(super) fn peer(&self) -> &Peer {
        &self.peer
    }

    pub(super) fn relocate(&mut self, new_offset: ByteOffset) {
        if self.at_offset == new_offset {
            return;
        }
        todo!();
    }
}
