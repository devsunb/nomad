use nvim_oxi::api;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Clear;

impl Clear {
    pub(crate) fn emit(self) {
        api::echo(
            core::iter::empty::<(&str, Option<&str>)>(),
            false,
            &api::opts::EchoOpts::default(),
        )
        .expect("all the arguments are valid");
    }
}
