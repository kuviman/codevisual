use ::*;

extern "C" {
    pub fn emscripten_set_element_css_size(
        target: *const c_char,
        width: c_double,
        height: c_double,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_get_element_css_size(
        target: *const c_char,
        width: *mut c_double,
        height: *mut c_double,
    ) -> EMSCRIPTEN_RESULT;
}
