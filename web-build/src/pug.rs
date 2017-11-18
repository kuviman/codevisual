use ::*;

pub fn compile_pug(src: &Path, dst: &Path) {
    let dst = Path::new(&std::env::var("OUT_DIR").unwrap()).join(dst);

    assert!(command(&format!("pug {} --out {}",
                             src.to_str().unwrap(),
                             dst.parent().unwrap().to_str().unwrap()))
        .status()
        .expect("Could not compile Pug")
        .success(), "Could not compile Pug");
}