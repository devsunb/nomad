/// TODO: docs
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BufferId(u64);

impl From<&nvim::api::Buffer> for BufferId {
    #[inline]
    fn from(_buffer: &nvim::api::Buffer) -> Self {
        todo!();
    }
}
