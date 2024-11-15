use nvimx::fs::{AbsPath, AbsPathBuf, FsNodeName};
use nvimx::plugin::Plugin;

/// TODO: docs.
#[derive(Default)]
pub struct Nomad;

impl Plugin for Nomad {
    const AUGROUP_NAME: &'static str = "nomad";
    const COMMAND_NAME: &'static str = "Mad";
    const DIAGNOSTIC_NAME: &'static str = "nomad";
    const NAMESPACE_NAME: &'static str = "nomad-namespace";

    fn log_dir(&self) -> AbsPathBuf {
        #[cfg(target_family = "unix")]
        {
            let mut home = match home::home_dir() {
                Some(home) if !home.as_os_str().is_empty() => {
                    <&AbsPath>::try_from(&*home)
                        .expect("home is absolute")
                        .to_owned()
                },
                _ => panic!("failed to get the home directory"),
            };
            home.push(<&FsNodeName>::try_from(".local").expect("it's valid"))
                .push(<&FsNodeName>::try_from("share").expect("it's valid"))
                .push(<&FsNodeName>::try_from("nvim").expect("it's valid"))
                .push(<&FsNodeName>::try_from("nomad").expect("it's valid"))
                .push(<&FsNodeName>::try_from("logs").expect("it's valid"));
            home
        }
        #[cfg(not(target_family = "unix"))]
        {
            unimplemented!()
        }
    }
}
