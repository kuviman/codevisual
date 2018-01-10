use ::*;

pub fn build(options: &Options) {
    let target_name = if let Some(ref example) = options.example {
        example.clone()
    } else if let Some(ref package) = options.package {
        package.clone()
    } else {
        Path::new(&options.path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .into()
    };

    // TODO: fixed memory should be better
    std::env::set_var("EMMAKEN_CFLAGS", "-s ALLOW_MEMORY_GROWTH=1");
    let cargo_target = format!("{}-unknown-emscripten", options.target);

    if let Some(ref emsdk) = options.emsdk {
        #[cfg(windows)]
        let env_output = Command::new("cmd")
            .arg("/C")
            .arg(format!("{}\\emsdk_env.bat", emsdk))
            .output()
            .unwrap();
        #[cfg(not(windows))]
        let env_output = Command::new("bash").arg(emsdk).output().unwrap();

        assert!(env_output.status.success());
        let env_output = env_output.stdout;

        let mut path: Vec<std::path::PathBuf> =
            std::env::split_paths(&std::env::var("PATH").unwrap()).collect();

        use std::io::BufRead;
        for line in env_output.lines() {
            let line: String = line.unwrap();
            if line.contains("PATH +=") {
                path.push(line.trim_left_matches("PATH +=").trim().into());
            } else if line.contains('=') {
                let mut parts = line.split('=');
                let name = parts.next().unwrap();
                let value = parts.next().unwrap();
                assert!(parts.next().is_none());
                std::env::set_var(name, value);
            }
        }
        std::env::set_var("PATH", std::env::join_paths(path).unwrap());
    }

    let mut command = Command::new("cargo");
    command.arg("build");
    command.arg("--color=always");
    if let Some(ref package) = options.package {
        command.arg("--package");
        command.arg(package);
    }
    if let Some(ref example) = options.example {
        command.arg("--example");
        command.arg(example);
    }
    if options.release {
        command.arg("--release");
    }
    command.arg(format!("--target={}", cargo_target));
    assert!(command.status().unwrap().success());

    // Copy resources
    use std::io::Write;
    let cargo_config = if options.release { "release" } else { "debug" };
    let build_dir = Path::new("target").join(cargo_target).join(cargo_config);
    let target_dir = Path::new("target").join("web").join(&options.target);
    fs::create_dir_all(&target_dir).unwrap();
    File::create(target_dir.join("codevisual.html"))
        .unwrap()
        .write_all(codevisual_web::HTML.as_ref())
        .unwrap();
    File::create(target_dir.join("codevisual.css"))
        .unwrap()
        .write_all(codevisual_web::CSS.as_ref())
        .unwrap();
    File::create(target_dir.join("codevisual.js"))
        .unwrap()
        .write_all(codevisual_web::JS.as_ref())
        .unwrap();
    fn copy_dir_contents<P, Q>(source: P, target: Q)
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let entries = fs::read_dir(source)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        fs_extra::copy_items(&entries, target, &options).unwrap();
    }
    copy_dir_contents(codevisual_web::STATIC_PATH, &target_dir);
    if let Some(ref path) = options.static_path {
        copy_dir_contents(path, &target_dir);
    }
    let target_js = if let Some(_) = options.example {
        build_dir
            .join("examples")
            .join(format!("{}.js", target_name))
    } else {
        build_dir.join(format!("{}.js", target_name))
    };
    fs::copy(target_js, target_dir.join("code.js")).unwrap();
    if options.target == "wasm32" {
        let mut target_wasm: Option<std::path::PathBuf> = None;
        let wasm_dir = if let Some(_) = options.example {
            "examples"
        } else {
            "deps"
        };
        for entry in fs::read_dir(build_dir.join(wasm_dir)).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if ext == "wasm" && file_name.starts_with(&target_name) {
                        assert!(
                            target_wasm.is_none(),
                            "Multiple .wasm files ({:?} and {:?})",
                            target_wasm.unwrap().file_name().unwrap(),
                            file_name
                        );
                        target_wasm = Some(path.clone());
                    }
                }
            }
        }
        let target_wasm = target_wasm.unwrap();
        fs::copy(
            &target_wasm,
            target_dir.join(target_wasm.file_name().unwrap()),
        ).unwrap();
    }
}
