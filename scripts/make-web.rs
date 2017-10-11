use std::process::Command;

fn main() {
    let mut command = Command::new("cargo");
    command.arg("build").arg("--release")
        .arg("--target=asmjs-unknown-emscripten");
    assert!(command.status().unwrap().success());
}