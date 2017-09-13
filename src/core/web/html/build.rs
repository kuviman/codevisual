extern crate web_build;

use std::path::Path;

fn main() {
    web_build::compile_pug(Path::new("src").join("lib.pug"), "lib.html");
}
