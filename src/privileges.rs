use std::env;
use std::process::Command;
use std::os::unix::process::CommandExt;

use nix::unistd::Uid;

use errors::*;

pub fn require_root() -> Result<()> {
    if !Uid::effective().is_root() {
        info!("Relaunching with 'sudo'");

        Err(Command::new("sudo").args(env::args()).exec()).chain_err(|| ErrorKind::SudoRelaunch)
    } else {
        Ok(())
    }
}
