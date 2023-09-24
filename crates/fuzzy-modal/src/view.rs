use common::*;

use crate::*;

pub(crate) struct View {
    prompt: Prompt,
    // results: Results,
}

impl View {
    pub fn new(config: FuzzyConfig, window_config: WindowConfig) -> Self {
        let FuzzyConfig {
            items,
            on_confirm,
            on_cancel,
            on_select,
            starting_text,
            starting_selected,
        } = config;

        let (prompt_config, _) = window_config.bisect_vertical(1);

        let len = items.len();

        let prompt = Prompt::new(
            starting_text.clone(),
            prompt_config,
            items.len() as _,
            move |query| {
                nvim::print!("new query is {query}");
                len as _
            },
        );

        Self { prompt }
    }

    pub fn close(self) {
        self.prompt.close();
        // self.results.close();
    }
}
