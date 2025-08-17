use core::fmt;
use std::collections::HashSet;

use abs_path::{AbsPathBuf, node, path};
use ed::fs::Directory;
use ed::fs::os::OsFs;
use tempdir::TempDir;
use thread_pool::ThreadPool;
use walkdir::GitIgnore;

#[test]
#[cfg_attr(not(git_in_PATH), ignore = "git is not in $PATH")]
fn gitignore_1() {
    let repo: TempDir = GitRepository::create_and_init(mock::fs! {
        "a.txt": "",
        "b.txt": "",
        ".gitignore": "a.txt",
    });

    assert_eq!(
        repo.non_ignored_paths().remove_git_dir(),
        ["/b.txt", "/.gitignore"]
    );
}

#[test]
#[cfg_attr(not(git_in_PATH), ignore = "git is not in $PATH")]
fn gitignore_2() {
    let repo: TempDir = GitRepository::create_and_init(mock::fs! {
        "a.txt": "",
        "b.txt": "",
        ".gitignore": "a.txt",
    });

    // Change the .gitignore file.
    std::fs::write(repo.path().join(node!(".gitignore")), "b.txt").unwrap();

    // Now 'b.txt' should be ignored, and 'a.txt' should not.
    assert_eq!(
        repo.non_ignored_paths().remove_git_dir(),
        ["/a.txt", "/.gitignore"]
    );
}

trait GitRepository: Directory {
    /// Creates a directory from the given [`mock::fs::MockFs`].
    ///
    /// Note that the returned directory will not be initialized as a Git
    /// repository. To do so, call [`Self::init`] or [`Self::create_and_init`].
    fn create(fs: mock::fs::MockFs) -> Self;

    /// Same as [`Self::create`] followed by [`Self::init`].
    fn create_and_init(fs: mock::fs::MockFs) -> Self {
        let repo = Self::create(fs);
        repo.init();
        repo
    }

    /// `git init`s the repository.
    fn init(&self);

    /// Returns the paths of all non-gitignored files and directories in the
    /// repository, relative to its root.
    fn non_ignored_paths(&self) -> NonIgnoredPaths {
        let mut thread_pool = ThreadPool::default();
        let ignore = GitIgnore::new(&self.path(), &mut thread_pool).unwrap();
        self.non_ignored_paths_with_gitignore(&ignore)
    }

    /// Same as [`Self::non_ignored_paths`], but uses the given [`GitIgnore`]
    /// instance instead of creating a new one.
    fn non_ignored_paths_with_gitignore(
        &self,
        gitignore: &GitIgnore,
    ) -> NonIgnoredPaths;
}

impl GitRepository for TempDir {
    fn create(fs: mock::fs::MockFs) -> Self {
        use tempdir::FsExt;

        futures_lite::future::block_on(async move {
            let tempdir = OsFs::default()
                .tempdir()
                .await
                .expect("couldn't create tempdir");

            tempdir
                .replicate_from(&fs.root())
                .await
                .expect("couldn't replicate from mock fs");

            tempdir
        })
    }

    fn init(&self) {
        use std::process::{Command, Stdio};
        Command::new("git")
            .arg("init")
            .current_dir(self.path())
            // Ignore all global config files.
            //
            // See https://stackoverflow.com/a/67512433 for more info.
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("failed to `git init` directory");
    }

    fn non_ignored_paths_with_gitignore(
        &self,
        gitignore: &GitIgnore,
    ) -> NonIgnoredPaths {
        use futures_lite::StreamExt;
        use walkdir::FsExt;

        futures_lite::future::block_on(async move {
            NonIgnoredPaths {
                inner: OsFs::default()
                    .walk(self)
                    .filter(gitignore)
                    .paths()
                    .map(Result::unwrap)
                    .map(|path| {
                        path.strip_prefix(self.path()).unwrap().to_owned()
                    })
                    .collect::<HashSet<_>>()
                    .await,
            }
        })
    }
}

struct NonIgnoredPaths {
    inner: HashSet<AbsPathBuf>,
}

impl NonIgnoredPaths {
    /// Removes all the paths of files and directories in the `/.git`
    /// directory.
    fn remove_git_dir(mut self) -> Self {
        self.inner.retain(|path| !path.starts_with(path!("/.git")));
        self
    }
}

impl<Paths, Path> PartialEq<Paths> for NonIgnoredPaths
where
    Paths: IntoIterator<Item = Path> + Clone,
    Path: AsRef<str>,
{
    fn eq(&self, other: &Paths) -> bool {
        let other = other
            .clone()
            .into_iter()
            .map(|path| {
                path.as_ref().parse::<AbsPathBuf>().expect("invalid path")
            })
            .collect::<HashSet<_>>();
        self.inner == other
    }
}

impl fmt::Debug for NonIgnoredPaths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entries(self.inner.iter().map(AsRef::<str>::as_ref))
            .finish()
    }
}
