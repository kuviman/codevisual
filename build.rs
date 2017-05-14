use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    Command::new("tsc.cmd")
        .arg("--outFile")
        .arg(Path::new(&out_dir).join("codevisual-lib-full.js"))
        .current_dir(Path::new("src").join("js"))
        .status()
        .expect("Could not compile TypeScript");
    let js = Command::new("java")
        .arg("-jar")
        .arg("C:\\Programs\\bin\\closure-compiler.jar")
        .arg(Path::new(&out_dir).join("codevisual-lib-full.js"))
        .output()
        .expect("Could not minify JavaScript")
        .stdout;
    File::create(Path::new(&out_dir).join("codevisual-lib.js"))
        .expect("Could not create js file")
        .write_all(&js)
        .expect("Could not write js");

    let css = Command::new("lessc.cmd")
        .arg("--clean-css")
        .arg(Path::new("src").join("css").join("codevisual-lib.less"))
        .output()
        .expect("Could not complile Less")
        .stdout;
    File::create(Path::new(&out_dir).join("codevisual-lib.css"))
        .expect("Could not create css file")
        .write_all(&css)
        .expect("Could not write css");

    Command::new("pug.cmd")
        .arg(Path::new("src").join("html").join("codevisual-lib.pug"))
        .arg("--out")
        .arg(Path::new(&out_dir))
        .status()
        .expect("Could not compile Pug");
}