use crate::*;

#[derive(Default)]
pub(crate) struct OneDark;

impl Colorscheme for OneDark {
    const NAME: &'static str = "One Dark";

    fn palette(&self) -> Palette {
        Palette {
            foreground: hex!("#abb2bf"),
            background: hex!("#282c34"),
            string: hex!("#98c379"),
        }
    }
}

impl BuiltinColorscheme for OneDark {}

impl SyntaxColorscheme for OneDark {}

impl DiagnosticColorscheme for OneDark {}

impl LspColorscheme for OneDark {}

impl TreeSitterColorscheme for OneDark {}

impl NomadColorscheme for OneDark {}

impl TelescopeColorscheme for OneDark {}
