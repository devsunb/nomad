use crate::{colorscheme::Palette as ColorschemePalette, *};

#[derive(Default)]
pub(crate) struct AyuMirage;

impl ColorschemePalette for AyuMirage {
    const PALETTE: Palette = Palette {
        foreground: hex!("#252935"),
        background: hex!("#cccac3"),
        string: hex!("#ddfc90"),
    };
}

impl Colorscheme for AyuMirage {
    const NAME: &'static str = "Ayu Mirage";
}

impl BuiltinColorscheme for AyuMirage {}

impl SyntaxColorscheme for AyuMirage {}

impl DiagnosticColorscheme for AyuMirage {}

impl LspColorscheme for AyuMirage {}

impl TreeSitterColorscheme for AyuMirage {}

impl NomadColorscheme for AyuMirage {}

impl TelescopeColorscheme for AyuMirage {}
