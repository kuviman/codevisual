mod mouse;
pub use self::mouse::*;

mod wheel;
pub use self::wheel::*;

mod touch;
pub use self::touch::*;

mod webgl;
pub use self::webgl::*;

use ::*;

pub(crate) fn into_canvas_pos(x: c_long, y: c_long) -> (f64, f64) {
    let (x, y) = (x as f64, y as f64);
    let (css_width, css_height) = unsafe {
        let mut css_width: c_double = std::mem::uninitialized();
        let mut css_height: c_double = std::mem::uninitialized();
        emscripten_get_element_css_size(std::ptr::null(), &mut css_width, &mut css_height);
        (css_width as f64, css_height as f64)
    };
    let (width, height) = get_canvas_size();
    let (width, height) = (width as f64, height as f64);
    (x * width / css_width, y * height / css_height)
}