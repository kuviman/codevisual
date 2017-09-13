use ::*;

pub fn compile_pug<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let dst = Path::new(&std::env::var("OUT_DIR").unwrap()).join(dst);

    #[cfg(windows)]
    Command::new("cmd")
        .arg("/C")
        .arg(format!("pug {} --out {}",
                     src.to_str().unwrap(),
                     dst.parent().unwrap().to_str().unwrap()))
        .status()
        .expect("Could not compile Pug");

    #[cfg(not(windows))]
    Command::new("pug")
        .arg(src)
        .arg("--out")
        .arg(dst.parent().unwrap())
        .status()
        .expect("Could not compile Pug");
}