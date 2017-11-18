use ::*;

pub fn run(options: &Options) {
    let port = 8123; // thread_rng().gen_range(8000, 9000);

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        assert!(open::that(format!("http://127.0.0.1:{}/codevisual.html", port)).unwrap().success());
    });

    let mut server = Command::new("python");
    server.current_dir(Path::new("target").join("web").join(&options.target));
    server.arg("-m").arg("SimpleHTTPServer");
    server.arg(port.to_string());
    assert!(server.status().unwrap().success());
}