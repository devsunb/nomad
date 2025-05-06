use criterion::BenchmarkGroup;
use criterion::measurement::WallTime;
use ed::fs::Directory;

pub(crate) fn benches(group: &mut BenchmarkGroup<'_, WallTime>) {
    #[cfg(feature = "neovim-repo")]
    read_neovim::from_mock_fs(group);

    #[cfg(feature = "neovim-repo")]
    read_neovim::from_real_fs(group);
}

#[cfg(feature = "neovim-repo")]
mod read_neovim {
    use criterion::BenchmarkId;
    use ed::fs::os::{OsDirectory, OsFs};
    use ed::fs::{self, Fs};
    use futures_lite::future;
    use thread_pool::ThreadPool;
    use walkdir::{Filter, GitIgnore};

    use super::*;

    pub(super) fn from_mock_fs(group: &mut BenchmarkGroup<'_, WallTime>) {
        let fs = mock::fs! {};

        // Replicate the Neovim repo into the root of the mock filesystem.
        future::block_on(async {
            fs.root().replicate_from(&neovim_repo()).await.unwrap();
        });

        bench_read_project(fs.root(), (), fs, "mock_fs", group);
    }

    pub(super) fn from_real_fs(group: &mut BenchmarkGroup<'_, WallTime>) {
        let neovim_repo = neovim_repo();
        let git_ignore = GitIgnore::new(neovim_repo.path().to_owned());
        bench_read_project(
            neovim_repo,
            git_ignore,
            OsFs::default(),
            "real_fs",
            group,
        );
    }

    fn neovim_repo() -> OsDirectory {
        future::block_on(async {
            OsFs::default()
                .node_at_path(crate::generated::collab::NEOVIM_REPO_PATH)
                .await
                .unwrap()
                .unwrap()
                .unwrap_directory()
        })
    }

    fn bench_read_project<Fs: fs::Fs>(
        project_root: Fs::Directory,
        filter: impl Filter<Fs>,
        fs: Fs,
        fs_name: &str,
        group: &mut BenchmarkGroup<'_, WallTime>,
    ) {
        let bench_id = BenchmarkId::new(
            "start",
            format_args!("read_neovim_from_{fs_name}"),
        );

        group.bench_function(bench_id, |b| {
            b.iter(|| {
                future::block_on(collab::start::benches::read_project(
                    project_root,
                    filter,
                    fs,
                    ThreadPool::new(),
                ))
                .unwrap()
            });
        });
    }
}
