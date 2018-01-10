#![deny(warnings)]

#[allow(unused_imports)]
pub(crate) use std::io::Write;
pub(crate) use std::fs::File;
pub(crate) use std::path::Path;
pub(crate) use std::process::Command;

mod less;
mod pug;
mod ts;

pub use less::*;
pub use pug::*;
pub use ts::*;

fn command(cmd: &str) -> Command {
    #[cfg(windows)]
    return {
        let mut command = Command::new("cmd");
        command.arg("/C").arg(cmd);
        command
    };
    #[cfg(not(windows))]
    return {
        let mut command = Command::new("sh");
        command.arg("-c").arg(cmd);
        command
    };
}
