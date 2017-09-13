use ::*;

pub fn compile_less<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let dst = Path::new(&std::env::var("OUT_DIR").unwrap()).join(dst);

    #[cfg(windows)]
    let css = Command::new("cmd")
        .arg("/C")
        .arg(format!("lessc --clean-css {}", src.to_str().unwrap()))
        .output()
        .expect("Could not complile Less")
        .stdout;

    #[cfg(not(windows))]
    let css = Command::new("lessc")
        .arg("--clean-css")
        .arg(src)
        .output()
        .expect("Could not complile Less")
        .stdout;

    File::create(dst)
        .expect("Could not create css file")
        .write_all(&css)
        .expect("Could not write css");
}