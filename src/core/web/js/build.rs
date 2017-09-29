extern crate web_build;

use std::path::Path;

fn main() {
    web_build::compile_ts(&Path::new("src"), &Path::new("lib.js"));
}
