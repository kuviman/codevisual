extern crate web_build;

use std::path::Path;

fn main() {
    web_build::compile_ts("src", "lib.js");
}
