//! TODO: docs.

#![feature(min_specialization)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod access;
pub mod action;
mod agent_id;
mod api;
mod buffer;
mod byte_offset;
pub mod command;
mod context;
mod cursor;
mod editor;
mod editor_adapter;
mod event_handle;
pub mod module;
pub mod notify;
pub mod plugin;
mod selection;
pub mod shared;
mod state;
mod util;

pub use access::{Access, AccessMut};
pub use agent_id::AgentId;
pub use api::{Api, ApiValue, Key, MapAccess, Value};
pub use buffer::{Buffer, Chunks, Edit, Replacement};
pub use byte_offset::ByteOffset;
pub use context::{BorrowState, Borrowed, Context, NotBorrowed};
pub use cursor::Cursor;
pub use editor::Editor;
pub use editor_adapter::EditorAdapter;
pub use event_handle::EventHandle;
pub use selection::Selection;
pub use shared::Shared;
