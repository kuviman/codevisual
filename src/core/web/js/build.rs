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

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    compile_js(&Path::new("src"), &Path::new(&out_dir).join("lib.js"));
}