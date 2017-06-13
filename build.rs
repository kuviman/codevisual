use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::Command;

#[cfg(windows)]
fn compile_js(source: &Path, out: &Path) {
    let full_js_file = Path::new(&std::env::var("OUT_DIR").unwrap()).join("codevisual-lib-full.js");
    assert!(Command::new("tsc.cmd")
                .arg("--outFile")
                .arg(&full_js_file)
                .current_dir(source)
                .status()
                .expect("Could not compile TypeScript")
                .success(),
            "TypeScript compiler exited with error");
    let js = {
        let result = Command::new("java")
            .arg("-jar")
            .arg("C:\\Programs\\bin\\closure-compiler.jar")
            .arg(&full_js_file)
            .output()
            .expect("Could not minify JavaScript");
        assert!(result.status.success(), "Could not minify JavaScript");
        result.stdout
    };
    File::create(out)
        .expect("Could not create js file")
        .write_all(&js)
        .expect("Could not write js");
}

#[cfg(not(windows))]
fn compile_js(source: &Path, out: &Path) {
    let full_js_file = Path::new(&std::env::var("OUT_DIR").unwrap()).join("codevisual-lib-full.js");
    assert!(Command::new("tsc")
                .arg("--outFile")
                .arg(&full_js_file)
                .current_dir(source)
                .status()
                .expect("Could not compile TypeScript")
                .success(),
            "TypeScript compiler exited with error");
    let js = {
        let result = Command::new("google-closure-compiler-js")
            .arg(&full_js_file)
            .output()
            .expect("Could not minify JavaScript");
        assert!(result.status.success(), "Could not minify JavaScript");
        result.stdout
    };
    File::create(out)
        .expect("Could not create js file")
        .write_all(&js)
        .expect("Could not write js");
}

fn compile_css(source: &Path, out: &Path) {
    #[cfg(windows)]
    let lessc_command = "lessc.cmd";
    #[cfg(not(windows))]
    let lessc_command = "lessc";

    let css = Command::new(lessc_command)
        .arg("--clean-css")
        .arg(source)
        .output()
        .expect("Could not complile Less")
        .stdout;
    File::create(out)
        .expect("Could not create css file")
        .write_all(&css)
        .expect("Could not write css");
}

fn compile_html(source: &Path, out_dir: &Path) {
    #[cfg(windows)]
    let pug_command = "pug.cmd";
    #[cfg(not(windows))]
    let pug_command = "pug";

    Command::new(pug_command)
        .arg(source)
        .arg("--out")
        .arg(Path::new(&out_dir))
        .status()
        .expect("Could not compile Pug");
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    compile_js(&Path::new("src").join("js"),
               &Path::new(&out_dir).join("codevisual-lib.js"));
    compile_css(&Path::new("src").join("css").join("codevisual-lib.less"),
                &Path::new(&out_dir).join("codevisual-lib.css"));
    compile_html(&Path::new("src").join("html").join("codevisual-lib.pug"),
                 &Path::new(&out_dir));
}