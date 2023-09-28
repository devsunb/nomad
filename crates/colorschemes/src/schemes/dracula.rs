use crate::*;

#[derive(Default)]
pub(crate) struct Dracula;

impl Colorscheme for Dracula {
    const NAME: &'static str = "Dracula";

    fn palette(&self) -> Palette {
        Palette {
            foreground: hex!("#f8f8f2"),
            background: hex!("#282a36"),
            string: hex!("#50fa7b"),
        }
    }
}

impl BuiltinColorscheme for Dracula {}

impl SyntaxColorscheme for Dracula {}

impl DiagnosticColorscheme for Dracula {}

impl LspColorscheme for Dracula {}

impl TreeSitterColorscheme for Dracula {}

impl NomadColorscheme for Dracula {}

impl TelescopeColorscheme for Dracula {}
