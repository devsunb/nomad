use nomad::prelude::{Get, Shared};

use crate::{Activity, Config};

/// TODO: docs
pub(crate) struct Context {
    pub(crate) activity: Shared<Activity>,
    pub(crate) config: Get<Config>,
}

impl Context {
    pub(crate) fn new(config: Get<Config>) -> Self {
        let activity = Shared::new(Activity::default());
        Self { activity, config }
    }
}
