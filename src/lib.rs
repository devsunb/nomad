#[nvim_oxi::module]
fn nomad() -> nvim_oxi::Dictionary {
    nomad::Nomad::new()
        .with_tracing_subscriber(tracing_subscriber::subscriber())
        .init()
        .api()
}
