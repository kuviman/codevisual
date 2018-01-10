use ::*;

pub fn compile_less(src: &Path, dst: &Path) {
    let dst = Path::new(&std::env::var("OUT_DIR").unwrap()).join(dst);

    let css = command(&format!("lessc --clean-css {}", src.to_str().unwrap()))
        .output()
        .expect("Could not complile Less")
        .stdout;

    File::create(dst)
        .expect("Could not create css file")
        .write_all(&css)
        .expect("Could not write css");
}
