use ::*;

#[cfg(windows)]
pub fn compile_ts<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D, dst_dts: Option<D>) {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir).join(dst);

    let full_js_file = Path::new(&out_dir).join("full.js");
    assert!(
        Command::new("cmd")
            .arg("/C")
            .arg(format!("tsc --declaration --outFile {}", full_js_file.to_str().unwrap()))
            .current_dir(src)
            .status()
            .expect("Could not compile TypeScript")
            .success(),
        "TypeScript compiler exited with error"
    );
    let js = {
        let result = Command::new("cmd")
            .arg("/C")
            .arg(format!("google-closure-compiler-js {}", full_js_file.to_str().unwrap()))
            .output()
            .expect("Could not minify JavaScript");
        assert!(result.status.success(), "Could not minify JavaScript");
        result.stdout
    };
    File::create(dst)
        .expect("Could not create js file")
        .write_all(&js)
        .expect("Could not write js");
    std::fs::remove_file(full_js_file).expect("Could not delete temp full js file");
    let dts_file = Path::new(&out_dir).join("full.d.ts");
    if let Some(dst_dts) = dst_dts {
        let dst_dts = dst_dts.as_ref();
        let dst_dts = Path::new(&dst_dts);
        if let Some(dst_dir) = dst_dts.parent() {
            std::fs::create_dir_all(dst_dir)
                .expect("Could not create declaration directory");
        }
        std::fs::copy(&dts_file, dst_dts)
            .expect("Could not copy declaration file");
    }
    std::fs::remove_file(&dts_file).expect("Could not remove declaration file");
}

#[cfg(not(windows))]
pub fn compile_ts<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir).join(dst);

    let full_js_file = Path::new(&out_dir).join("full.js");
    assert!(
        Command::new("tsc")
            .arg("--declaration")
            .arg("--outFile")
            .arg(&full_js_file)
            .current_dir(src)
            .status()
            .expect("Could not compile TypeScript")
            .success(),
        "TypeScript compiler exited with error"
    );
    let js = {
        let result = Command::new("google-closure-compiler-js")
            .arg(&full_js_file)
            .output()
            .expect("Could not minify JavaScript");
        assert!(result.status.success(), "Could not minify JavaScript");
        result.stdout
    };
    File::create(dst)
        .expect("Could not create js file")
        .write_all(&js)
        .expect("Could not write js");
    std::fs::remove_file(full_js_file).expect("Could not delete temp full js file");
    let dts_file = Path::new(&out_dir).join("full.d.ts");
    if let Some(dst_dts) = dst_dts {
        let dst_dts = dst_dts.as_ref();
        let dst_dts = Path::new(&dst_dts);
        if let Some(dst_dir) = dst_dts.parent() {
            std::fs::create_dir_all(dst_dir)
                .expect("Could not create declaration directory");
        }
        std::fs::copy(&dts_file, dst_dts)
            .expect("Could not copy declaration file");
    }
    std::fs::remove_file(&dts_file).expect("Could not remove declaration file");
}