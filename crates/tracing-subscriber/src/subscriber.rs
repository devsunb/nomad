use std::env;
use std::path::PathBuf;

const LOG_FILE_NAME: &str = "nomad.log";

/// TODO: docs
pub struct Subscriber;

impl Subscriber {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl tracing::Subscriber {
        let file_appender =
            tracing_appender::rolling::never(log_dir(), LOG_FILE_NAME);

        let (non_blocking, _guard) =
            tracing_appender::non_blocking(file_appender);

        tracing_subscriber::fmt().with_writer(non_blocking).finish()
    }
}

#[cfg(unix)]
fn log_dir() -> PathBuf {
    if let Ok(nomad_log_dir) = env::var("NOMAD_LOG_DIR") {
        PathBuf::from(nomad_log_dir)
    } else if let Ok(xdg_state_home) = env::var("XDG_STATE_HOME") {
        PathBuf::from(xdg_state_home).join("nomad")
    } else if let Some(home_dir) = home::home_dir() {
        home_dir.join(".local").join("state").join("nomad")
    } else {
        panic!("Could not determine log directory");
    }
}

#[cfg(windows)]
fn log_dir() -> PathBuf {
    todo!();
}
