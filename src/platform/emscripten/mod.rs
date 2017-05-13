pub mod ffi;

pub fn init() {
    ffi::eval_js(include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.js")));
    ffi::call_js("CodeVisual.ffi.init_css", include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.css")));
    ffi::call_js("CodeVisual.ffi.init_html", include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.html")));
}