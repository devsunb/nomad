use std::io;

use collab_fs::{AbsUtf8Path, AbsUtf8PathBuf, Fs};
use futures_util::{pin_mut, StreamExt};

use crate::Marker;

/// TODO: docs.
pub struct Finder;

impl Finder {
    /// TODO: docs.
    pub async fn find_root<T: Marker, F: Fs>(
        &mut self,
        start_from: &AbsUtf8Path,
        marker: &T,
        fs: &F,
    ) -> io::Result<Option<AbsUtf8PathBuf>> {
        let mut dir = match start_from.parent() {
            Some(dir) => dir.to_owned(),
            None => {
                let dir = AbsUtf8PathBuf::root();
                debug_assert_eq!(start_from, &*dir);
                return contains_marker(&dir, marker, fs)
                    .await
                    .map(|contains| contains.then_some(dir));
            },
        };

        loop {
            if contains_marker(&dir, marker, fs).await? {
                return Ok(Some(dir));
            }
            if !dir.pop() {
                return Ok(None);
            }
        }
    }
}

async fn contains_marker(
    dir: &AbsUtf8Path,
    marker: &impl Marker,
    fs: &impl Fs,
) -> io::Result<bool> {
    let entries = fs.read_dir(dir).await?;
    pin_mut!(entries);

    let mut dir = dir.to_owned();
    while let Some(entry) = entries.next().await {
        let file_name = fs.file_name(&entry).await?;
        dir.push(file_name.as_str());
        let metadata = fs.metadata(&entry).await?;
        if marker.matches(&dir, &metadata, fs).await? {
            return Ok(true);
        }
        dir.pop();
    }

    Ok(false)
}
