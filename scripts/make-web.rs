extern crate prelude;
extern crate open;

use prelude::*;

use std::process::Command;

fn build(release: bool, target: &str) {
    let cargo_target = format!("{}-unknown-emscripten", target);
    let mut command = Command::new("cargo");
    command.arg("build");
    if release {
        command.arg("--release");
    }
    command.arg(format!("--target={}", cargo_target));
    assert!(command.status().unwrap().success());
}

fn main() {
    let release = true;
    let target = "asmjs";
    build(release, target);

    let port = thread_rng().gen_range(8000, 9000);

    assert!(open::that(format!("http://127.0.0.1:{}", port)).unwrap().success());

    let mut server = Command::new("simple-http-server");
    server.arg("--ip").arg("127.0.0.1");
    server.arg("-p").arg(port.to_string());
    assert!(server.status().unwrap().success());
}