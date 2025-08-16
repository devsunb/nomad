//! TODO: docs.

mod file_appender;
mod nomad;

#[neovim::plugin]
fn nomad() -> nomad::Nomad {
    nomad::Nomad
}
