use crate::*;

pub(crate) struct AyuMirage {
    foreground: Color,
    background: Color,
}

impl Default for AyuMirage {
    fn default() -> Self {
        Self { foreground: hex!("#252935"), background: hex!("#cccac3") }
    }
}

impl Colorscheme for AyuMirage {
    const NAME: &'static str = "Ayu Mirage";
}

impl BaseColorscheme for AyuMirage {
    fn normal(&self) -> Option<HighlightGroup> {
        HighlightGroup::new()
            .with_foreground(self.foreground)
            .with_background(self.background)
            .into_some()
    }
}

impl DiagnosticColorscheme for AyuMirage {}

impl LspColorscheme for AyuMirage {}

impl TreeSitterColorscheme for AyuMirage {}

impl NomadColorscheme for AyuMirage {}

impl TelescopeColorscheme for AyuMirage {}
