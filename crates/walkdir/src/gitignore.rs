use core::time::Duration;
use std::sync::Mutex;
use std::time::Instant;

use ed::fs::{AbsPath, AbsPathBuf, Metadata, NodeName, os};

use crate::{DirEntry, Either, Filter};

/// TODO: docs.
pub struct GitIgnore {
    dir_path: AbsPathBuf,
    ignored_paths: Mutex<(IgnoredPaths, Instant)>,
}

/// TODO: docs.
#[derive(Debug, thiserror::Error)]
pub enum GitIgnoreError {}

#[derive(Default)]
struct IgnoredPaths {
    _inner: Vec<AbsPathBuf>,
}

impl GitIgnore {
    /// TODO: docs.
    const REFRESH_IGNORED_PATHS_AFTER: Duration = Duration::from_secs(10);

    /// TODO: docs.
    pub fn new(dir_path: AbsPathBuf) -> Self {
        let outdated_time = Instant::now()
            - Self::REFRESH_IGNORED_PATHS_AFTER
            - Duration::from_secs(1);

        Self {
            dir_path,
            ignored_paths: Mutex::new((
                IgnoredPaths::default(),
                outdated_time,
            )),
        }
    }

    fn with_ignored<R>(
        &self,
        f: impl FnOnce(&IgnoredPaths) -> R,
    ) -> Result<R, GitIgnoreError> {
        let (paths, last_refreshed_ignored_paths_at) =
            &mut *self.ignored_paths.lock().expect("poisoned mutex");

        if Instant::now() - *last_refreshed_ignored_paths_at
            > Self::REFRESH_IGNORED_PATHS_AFTER
        {
            *paths = IgnoredPaths::get(&self.dir_path)?;
            *last_refreshed_ignored_paths_at = Instant::now();
        }

        Ok(f(paths))
    }
}

impl IgnoredPaths {
    fn contains(&self, _path: &impl PartialEq<AbsPath>) -> bool {
        todo!()
    }

    fn get(_dir_path: &AbsPath) -> Result<Self, GitIgnoreError> {
        todo!()
    }
}

// We're shelling out to Git to get the list of ignored files, so this can only
// be a filter on a real filesystem.
impl Filter<os::OsFs> for GitIgnore {
    type Error =
        Either<<DirEntry<os::OsFs> as Metadata>::NameError, GitIgnoreError>;

    async fn should_filter(
        &self,
        dir_path: &AbsPath,
        entry: &DirEntry<os::OsFs>,
    ) -> Result<bool, Self::Error> {
        struct Concat<'a>(&'a AbsPath, &'a NodeName);

        impl PartialEq<AbsPath> for Concat<'_> {
            fn eq(&self, path: &AbsPath) -> bool {
                let &Self(parent, name) = self;
                parent.len() + name.len() + 1 == path.len()
                    && parent == &path[..parent.len()]
                    && name == &path[parent.len() + 1..]
            }
        }

        let entry_name = entry.name().await.map_err(Either::Left)?;
        let path = Concat(dir_path, &entry_name);
        self.with_ignored(|ignored| ignored.contains(&path))
            .map_err(Either::Right)
    }
}
