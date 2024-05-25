use std::env;
use std::path::PathBuf;

/// TODO: docs
pub(crate) fn target_dir() -> PathBuf {
    todo!();
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum BuildProfile {
    Debug,
    Release,
}

impl BuildProfile {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Release => "release",
        }
    }

    pub(crate) fn from_env() -> Self {
        let profile = env::var("PROFILE").expect("$PROFILE env var not set");

        match profile.as_str() {
            "debug" => Self::Debug,
            "release" => Self::Release,
            _ => unreachable!("unknown profile {profile:?}"),
        }
    }
}
