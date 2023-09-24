use std::sync::Arc;

use common::nvim::{
    self,
    api::{opts::*, types::*, Buffer, Window},
};
use common::WindowConfig;

type OnQueryChange = Arc<dyn Fn(&str) -> u64 + 'static>;

/// TODO: docs
pub(crate) struct Prompt {
    current_text: String,
    default_text: Option<String>,
    matched_items: u64,
    total_items: u64,
    on_query_change: OnQueryChange,
    buffer: Buffer,
    window: Window,
}

impl Prompt {
    pub fn close(self) {
        todo!();
    }

    pub fn new<F>(
        default_text: Option<String>,
        window_config: WindowConfig,
        total_items: u64,
        on_query_change: F,
    ) -> Self
    where
        F: Fn(&str) -> u64 + 'static,
    {
        let (buffer, _text_extmark_id, _matched_items_extmark_id) =
            open_buffer(default_text.as_deref(), total_items);

        let window =
            nvim::api::open_win(&buffer, true, &((&window_config).into()))
                .unwrap();

        Self {
            buffer,
            window,
            current_text: String::new(),
            default_text,
            matched_items: total_items,
            total_items,
            on_query_change: Arc::new(on_query_change),
        }
    }
}

fn open_buffer(
    default_text: Option<&str>,
    total_items: u64,
) -> (Buffer, Option<u32>, u32) {
    let mut buffer = nvim::api::create_buf(false, true).unwrap();

    // Create an anonymous namespace for the prompt.
    let ns_id = nvim::api::create_namespace("");

    let text_extmark_id = default_text.map(|text| {
        create_extmark(&mut buffer, ns_id, ExtmarkPosition::Start, text, "")
    });

    let matched_items_extmark_id = {
        let text = format!("{}/{}", total_items, total_items);
        create_extmark(&mut buffer, ns_id, ExtmarkPosition::Start, &text, "")
    };

    (buffer, text_extmark_id, matched_items_extmark_id)
}

enum ExtmarkPosition {
    Start,
    End,
}

fn create_extmark(
    buffer: &mut Buffer,
    ns_id: u32,
    position: ExtmarkPosition,
    text: &str,
    hl_group: &'static str,
) -> u32 {
    let position = match position {
        ExtmarkPosition::Start => ExtmarkVirtTextPosition::Overlay,
        ExtmarkPosition::End => ExtmarkVirtTextPosition::RightAlign,
    };

    let opts = SetExtmarkOpts::builder()
        .virt_text([(text, hl_group)])
        .virt_text_pos(position)
        .build();

    buffer.set_extmark(ns_id, 0, 0, &opts).unwrap()
}
