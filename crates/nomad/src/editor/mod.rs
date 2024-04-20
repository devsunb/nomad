//! TODO: docs

mod buffer;
mod buffer_state;

pub use buffer::{Buffer, RemoteDeletion, RemoteInsertion};
use buffer_state::{BufferState, LocalDeletion, LocalInsertion};
