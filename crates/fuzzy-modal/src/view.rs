use common::*;

use crate::*;

pub(crate) struct View {
    prompt: Prompt,
    // results: Results,
}

impl View {
    pub fn new(sender: Sender<Message>) -> Self {
        Self { prompt: Prompt::new(sender) }
    }

    pub fn open(&mut self, config: FuzzyConfig, window_config: WindowConfig) {
        let FuzzyConfig {
            items,
            on_confirm,
            on_cancel,
            on_select,
            starting_text,
            starting_selected,
        } = config;

        let (prompt_config, _) = window_config.bisect_vertical(1);

        // self.prompt.open(prompt_config, window_config);
    }

    pub fn close(&mut self) {
        self.prompt.close();
        // self.results.close();
    }
}
