extern crate web_build;

use std::path::Path;

fn main() {
    web_build::compile_less(&Path::new("css").join("lib.less"), &Path::new("lib.css"));
    web_build::compile_pug(&Path::new("html").join("lib.pug"), &Path::new("lib.html"));
    web_build::compile_ts(&Path::new("js"), &Path::new("lib.js"));
}
