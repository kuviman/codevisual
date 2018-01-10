use ::*;

pub fn compile_ts(src: &Path, dst: &Path) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir).join(dst);

    let full_js_file = Path::new(&out_dir).join("full.js");
    let cmd = format!(
        "tsc --declaration --outFile {} -p {}",
        full_js_file.to_str().unwrap(),
        src.to_str().unwrap()
    );
    assert!(
        command(&cmd)
            .status()
            .expect("Could not compile TypeScript")
            .success(),
        "TypeScript compiler exited with error"
    );
    let js = {
        let result = command(&format!(
            "google-closure-compiler-js {}",
            full_js_file.to_str().unwrap()
        )).output()
            .expect("Could not minify JavaScript");
        assert!(result.status.success(), "Could not minify JavaScript");
        result.stdout
    };
    File::create(&dst)
        .expect("Could not create js file")
        .write_all(&js)
        .expect("Could not write js");
    std::fs::remove_file(full_js_file).expect("Could not delete temp full js file");
    let dts_file = Path::new(&out_dir).join("full.d.ts");
    let dst_dts = dst.with_extension("d.ts");
    std::fs::create_dir_all(dst_dts.parent().unwrap())
        .expect("Could not create declaration directory");
    std::fs::rename(&dts_file, dst_dts).expect("Could not move declaration file");
}
