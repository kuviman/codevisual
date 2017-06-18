use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    #[cfg(windows)]
    let lessc_command = "lessc.cmd";
    #[cfg(not(windows))]
    let lessc_command = "lessc";

    let css = Command::new(lessc_command)
        .arg("--clean-css")
        .arg(Path::new("src").join("lib.less"))
        .output()
        .expect("Could not complile Less")
        .stdout;

    File::create(Path::new(&out_dir).join("lib.css"))
        .expect("Could not create css file")
        .write_all(&css)
        .expect("Could not write css");
}