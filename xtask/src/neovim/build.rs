use core::{fmt, iter, str};
use std::{env, fs};

use abs_path::{AbsPath, AbsPathBuf, NodeNameBuf, node};
use anyhow::{Context, anyhow};
use cargo_metadata::TargetKind;
use xshell::cmd;

use crate::WORKSPACE_ROOT;
use crate::neovim::CARGO_TOML_META;

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct BuildArgs {
    /// Build the plugin in release mode.
    #[clap(long, short, default_value_t = false)]
    release: bool,

    /// Build the plugin for the latest nightly version of Neovim.
    #[clap(long, default_value_t = false)]
    nightly: bool,

    /// The absolute path to the directory under which to place the build
    /// artifacts.
    #[clap(long, default_value_t = WORKSPACE_ROOT.to_owned())]
    out_dir: AbsPathBuf,
}

pub(crate) fn build(args: BuildArgs) -> anyhow::Result<()> {
    let sh = xshell::Shell::new()?;

    fs::create_dir_all(&args.out_dir)?;

    let artifact_dir = args.out_dir.clone().join(node!("lua"));

    // Setting the artifact directory is still unstable.
    let artifact_dir_args = ["-Zunstable-options", "--artifact-dir"]
        .into_iter()
        .chain(iter::once(artifact_dir.as_str()));

    let package_meta = &CARGO_TOML_META;

    // Specify which package to build.
    let package_args = ["--package", &package_meta.name].into_iter();

    let is_nightly = args.nightly
        || NeovimVersion::detect(&sh).map(|v| v.is_nightly()).unwrap_or(false);

    let feature_args =
        is_nightly.then_some("--features=neovim-nightly").into_iter();

    let profile_args = args.release.then_some("--release").into_iter();

    let build_args = artifact_dir_args
        .chain(package_args)
        .chain(feature_args)
        .chain(profile_args);

    cmd!(sh, "cargo build {build_args...}").run()?;

    fix_library_name(&artifact_dir)?;

    Ok(())
}

#[allow(clippy::unwrap_used)]
fn fix_library_name(artifact_dir: &AbsPath) -> anyhow::Result<()> {
    let package_meta = &CARGO_TOML_META;

    let mut cdylib_targets = package_meta.targets.iter().filter(|target| {
        target.kind.iter().any(|kind| kind == &TargetKind::CDyLib)
    });

    let cdylib_target = cdylib_targets.next().ok_or_else(|| {
        anyhow!(
            "Could not find a cdylib target in manifest of package {:?}",
            package_meta.name
        )
    })?;

    if cdylib_targets.next().is_some() {
        return Err(anyhow!(
            "Found multiple cdylib targets in manifest of package {:?}",
            package_meta.name
        ));
    }

    let source = format!(
        "{prefix}{lib_name}{suffix}",
        prefix = env::consts::DLL_PREFIX,
        lib_name = &cdylib_target.name,
        suffix = env::consts::DLL_SUFFIX
    )
    .parse::<NodeNameBuf>()
    .unwrap();

    let dest = format!(
        "{lib_name}{suffix}",
        lib_name = &cdylib_target.name,
        suffix = if cfg!(target_os = "windows") { ".dll" } else { ".so" }
    )
    .parse::<NodeNameBuf>()
    .unwrap();

    force_rename(&artifact_dir.join(&source), &artifact_dir.join(&dest))
        .context("Failed to rename the library")
}

fn force_rename(src: &AbsPath, dst: &AbsPath) -> anyhow::Result<()> {
    if fs::metadata(dst).is_ok() {
        fs::remove_file(dst)?;
    }
    fs::rename(src, dst)?;
    Ok(())
}

/// The possible Neovim versions our plugin can be built for.
#[derive(Debug, Copy, Clone)]
enum NeovimVersion {
    /// The latest stable version.
    ZeroDotEleven,

    /// The latest nightly version.
    Nightly,
}

struct SemanticVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

impl NeovimVersion {
    fn detect(sh: &xshell::Shell) -> Option<Self> {
        let version = "--version";
        let stdout = cmd!(sh, "nvim {version}").read().ok()?;
        let (_, rest) = stdout.lines().next()?.split_once("NVIM v")?;
        rest.parse::<Self>().ok()
    }

    fn is_nightly(self) -> bool {
        matches!(self, Self::Nightly)
    }
}

impl str::FromStr for NeovimVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nightly_suffix = "-dev";
        let is_nightly = s.ends_with(nightly_suffix);
        let version = s
            [..s.len() - (is_nightly as usize) * nightly_suffix.len()]
            .parse::<SemanticVersion>()
            .context("Failed to parse Neovim version")?;
        if version.major == 0 && version.minor == 11 {
            Ok(Self::ZeroDotEleven)
        } else if version.major == 0 && version.minor == 12 && is_nightly {
            Ok(Self::Nightly)
        } else {
            Err(anyhow!(
                "Unsupported Neovim version: {version}{}",
                if is_nightly { nightly_suffix } else { "" }
            ))
        }
    }
}

impl fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl str::FromStr for SemanticVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let major =
            parts.next().ok_or_else(|| anyhow!("major version is missing"))?;
        let minor =
            parts.next().ok_or_else(|| anyhow!("minor version is missing"))?;
        let patch =
            parts.next().ok_or_else(|| anyhow!("patch version is missing"))?;
        if parts.next().is_some() {
            return Err(anyhow!("too many version parts"));
        }
        Ok(Self {
            major: major.parse()?,
            minor: minor.parse()?,
            patch: patch.parse()?,
        })
    }
}
