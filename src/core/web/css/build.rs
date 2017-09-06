use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    #[cfg(windows)]
    let css = Command::new("cmd")
        .arg("/C")
        .arg(format!("lessc --clean-css {}", Path::new("src").join("lib.less").to_str().unwrap()))
        .output()
        .expect("Could not complile Less")
        .stdout;

    #[cfg(not(windows))]
    let css = Command::new("lessc")
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
