extern crate web_build;

use std::path::Path;

fn main() {
    web_build::compile_ts("src", "lib.js", None);
    web_build::compile_pug("src/overlay.pug", "overlay.html");
    web_build::compile_less("src/overlay.less", "overlay.css");
}
