use std::process::Command;

fn main() {
    let mut command = Command::new("cargo");
    command.arg("doc").arg("--no-deps")
        .arg("-p").arg("prelude")
        .arg("-p").arg("ugli");
    assert!(command.status().unwrap().success());
    
    let mut command = Command::new("cargo");
    command.arg("doc").arg("--no-deps");
    if std::env::args().find(|arg| arg == "--open").is_some() {
        command.arg("--open");
    }
    assert!(command.status().unwrap().success());
}