use api::types::*;
use nvim::api;

use crate::{Bound, Cells, Render, RequestedBound, Scene};

/// TODO: docs.
pub(crate) struct View {
    /// TODO: docs.
    buffer: api::Buffer,

    /// TODO: docs.
    root: Box<dyn Render + 'static>,

    /// TODO: docs.
    scene: Scene,

    /// TODO: docs.
    window: api::Window,
}

impl View {
    #[inline]
    pub(crate) fn is_hidden(&self) -> bool {
        self.window
            .get_config()
            .map(|config| config.hide.unwrap_or(false))
            .unwrap_or(false)
    }

    /// Opens a new `View`.
    #[inline]
    pub(crate) fn open(
        root: Box<dyn Render + 'static>,
        available_size: Bound<Cells>,
    ) -> Self {
        let buffer = api::create_buf(false, true).expect("never fails(?)");

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .height(1)
            .width(1)
            .row(0)
            .col(0)
            .hide(true)
            .style(WindowStyle::Minimal)
            .build();

        let window = api::open_win(&buffer, false, &config)
            .expect("the config is valid");

        let mut this = Self { buffer, root, scene: Scene::new(), window };

        this.render(available_size);

        this
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn render(&mut self, available_size: Bound<Cells>) {
        let requested_size = self.root.layout();

        let size = match requested_size {
            RequestedBound::Explicit(size) => size.intersect(available_size),
            RequestedBound::Available => available_size,
        };

        self.scene.resize(size);

        let scene_fragment = self.scene.as_fragment();

        self.root.paint(scene_fragment);

        self.scene.diff().apply(self);
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn size(&self) -> Bound<Cells> {
        self.scene.size()
    }
}
