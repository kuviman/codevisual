use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    #[cfg(windows)]
    Command::new("cmd")
        .arg("/C")
        .arg(format!("pug {} --out {}",
                     Path::new("src").join("lib.pug").to_str().unwrap(),
                     Path::new(&out_dir).to_str().unwrap()))
        .status()
        .expect("Could not compile Pug");

    #[cfg(not(windows))]
    Command::new("pug")
        .arg(Path::new("src").join("lib.pug"))
        .arg("--out")
        .arg(Path::new(&out_dir))
        .status()
        .expect("Could not compile Pug");
}
