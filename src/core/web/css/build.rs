extern crate web_build;

use std::path::Path;

fn main() {
    web_build::build_less(Path::new("src").join("lib.less"), Path::new("lib.css"));
}
