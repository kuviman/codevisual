use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    #[cfg(windows)]
    let pug_command = "pug.cmd";
    #[cfg(not(windows))]
    let pug_command = "pug";

    Command::new(pug_command)
        .arg(Path::new("src").join("lib.pug"))
        .arg("--out")
        .arg(Path::new(&out_dir))
        .status()
        .expect("Could not compile Pug");
}