extern crate emscripten_sys;
extern crate serde;
extern crate serde_json;

use std::os::raw::{c_int, c_long, c_void, c_double};
use std::ffi::CString;

#[macro_export]
macro_rules! format_placeholders {
    () => ("");
    ($arg:expr) => ("{}");
    ($head:expr, $($tail:expr),+) => (
        concat!("{},", format_placeholders!($($tail),+))
    )
}

#[macro_export]
macro_rules! run_js {
    ($($($f:ident).+ ( $($args:expr),* );)*) => (
        $(
            $crate::run_script(&format!(
                concat!(stringify!($($f).+), "(", format_placeholders!($($args),*), ")"),
                $($crate::IntoJson::into_json($args)),*));
        )*
    )
}

pub trait IntoJson {
    fn into_json(self) -> String;
}

impl<'a, T: ?Sized + serde::Serialize> IntoJson for &'a T {
    fn into_json(self) -> String {
        ::serde_json::to_string(self).expect("Could not convert to JSON")
    }
}

pub fn random() -> f64 {
    unsafe { ::emscripten_sys::emscripten_random() as f64 }
}

pub fn get_now() -> f64 {
    unsafe { ::emscripten_sys::emscripten_get_now() / 1000.0 as f64 }
}

pub fn get_proc_address(name: &str) -> *const std::os::raw::c_void {
    unsafe {
        ::emscripten_sys::emscripten_GetProcAddress(CString::new(name)
                                      .expect("Could not convert name to C string")
                                      .as_ptr()) as *const _
    }
}

pub fn run_script(script: &str) {
    unsafe {
        ::emscripten_sys::emscripten_run_script(CString::new(script)
                                                    .expect("Could not convert script to C string",)
                                                    .as_ptr());
    }
}

pub fn get_canvas_size() -> (u32, u32) {
    use std::os::raw::c_int;
    unsafe {
        let mut width: c_int = std::mem::uninitialized();
        let mut height: c_int = std::mem::uninitialized();
        let mut is_fullscreen: c_int = std::mem::uninitialized();
        ::emscripten_sys::emscripten_get_canvas_size(&mut width, &mut height, &mut is_fullscreen);
        (width as u32, height as u32)
    }
}

pub fn create_gl_context() -> Result<(), String> {
    unsafe {
        let mut attributes: ::emscripten_sys::EmscriptenWebGLContextAttributes =
            std::mem::uninitialized();
        ::emscripten_sys::emscripten_webgl_init_context_attributes(&mut attributes);
        let context = ::emscripten_sys::emscripten_webgl_create_context(std::ptr::null(),
                                                                        &attributes);
        if context <= 0 {
            return Err(String::from("Could not create WebGL context"));
        }
        ::emscripten_sys::emscripten_webgl_make_context_current(context);
    }
    Ok(())
}

pub fn set_main_loop<F: FnMut()>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        ::emscripten_sys::emscripten_set_main_loop_arg(Some(wrapper::<F>),
                                                       Box::into_raw(callback) as *mut _,
                                                       0,
                                                       1);
        emscripten_sys::emscripten_GetProcAddress(std::ptr::null());
    }
    unsafe extern "C" fn wrapper<F>(arg: *mut c_void)
        where F: FnMut()
    {
        let mut callback = Box::<Box<F>>::from_raw(arg as *mut _);
        callback();
        std::mem::forget(callback);
    }
}

fn into_canvas_pos(x: c_long, y: c_long) -> (f64, f64) {
    let mut css_width: c_double = unsafe { std::mem::uninitialized() };
    let mut css_height: c_double = unsafe { std::mem::uninitialized() };
    unsafe {
        ::emscripten_sys::emscripten_get_element_css_size(std::ptr::null(),
                                                          &mut css_width,
                                                          &mut css_height);
    }
    let (width, height) = get_canvas_size();
    (x as f64 * width as f64 / css_width as f64, y as f64 * height as f64 / css_height as f64)
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub struct MouseDownEvent {
    pub canvas_x: f64,
    pub canvas_y: f64,
    pub button: MouseButton,
}

pub fn set_mousedown_callback<F: FnMut(MouseDownEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        ::emscripten_sys::emscripten_set_mousedown_callback(CString::new("#canvas")
                                                                .unwrap()
                                                                .as_ptr(),
                                                            Box::into_raw(callback) as *mut _,
                                                            1,
                                                            Some(wrapper::<F>));
    }
    unsafe extern "C" fn wrapper<F>(_: c_int,
                                    event: *const ::emscripten_sys::EmscriptenMouseEvent,
                                    arg: *mut c_void)
                                    -> c_int
        where F: FnMut(MouseDownEvent)
    {
        let event = *event;
        let mut callback = Box::<Box<F>>::from_raw(arg as *mut _);
        let (canvas_x, canvas_y) = into_canvas_pos(event.canvasX, event.canvasY);
        callback(MouseDownEvent {
                     canvas_x,
                     canvas_y,
                     button: match event.button {
                         0 => MouseButton::Left,
                         1 => MouseButton::Middle,
                         2 => MouseButton::Right,
                         _ => panic!("Unexpected mouse button pressed"),
                     },
                 });
        std::mem::forget(callback);
        1
    }
}

pub struct MouseUpEvent {
    pub canvas_x: f64,
    pub canvas_y: f64,
    pub button: MouseButton,
}

pub fn set_mouseup_callback<F: FnMut(MouseUpEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        ::emscripten_sys::emscripten_set_mouseup_callback(CString::new("#canvas")
                                                              .unwrap()
                                                              .as_ptr(),
                                                          Box::into_raw(callback) as *mut _,
                                                          1,
                                                          Some(wrapper::<F>));
    }
    unsafe extern "C" fn wrapper<F>(_: c_int,
                                    event: *const ::emscripten_sys::EmscriptenMouseEvent,
                                    arg: *mut c_void)
                                    -> c_int
        where F: FnMut(MouseUpEvent)
    {
        let event = *event;
        let mut callback = Box::<Box<F>>::from_raw(arg as *mut _);
        let (canvas_x, canvas_y) = into_canvas_pos(event.canvasX, event.canvasY);
        callback(MouseUpEvent {
                     canvas_x,
                     canvas_y,
                     button: match event.button {
                         0 => MouseButton::Left,
                         1 => MouseButton::Middle,
                         2 => MouseButton::Right,
                         _ => panic!("Unexpected mouse button pressed"),
                     },
                 });
        std::mem::forget(callback);
        1
    }
}

pub struct MouseMoveEvent {
    pub canvas_x: f64,
    pub canvas_y: f64,
}

pub fn set_mousemove_callback<F: FnMut(MouseMoveEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        ::emscripten_sys::emscripten_set_mousemove_callback(CString::new("#canvas")
                                                                .unwrap()
                                                                .as_ptr(),
                                                            Box::into_raw(callback) as *mut _,
                                                            1,
                                                            Some(wrapper::<F>));
    }
    unsafe extern "C" fn wrapper<F>(_: c_int,
                                    event: *const ::emscripten_sys::EmscriptenMouseEvent,
                                    arg: *mut c_void)
                                    -> c_int
        where F: FnMut(MouseMoveEvent)
    {
        let event = *event;
        let mut callback = Box::<Box<F>>::from_raw(arg as *mut _);
        let (canvas_x, canvas_y) = into_canvas_pos(event.canvasX, event.canvasY);
        callback(MouseMoveEvent { canvas_x, canvas_y });
        std::mem::forget(callback);
        1
    }
}

pub struct WheelEvent {
    pub canvas_x: f64,
    pub canvas_y: f64,
    pub delta: f64,
}

pub fn set_wheel_callback<F: FnMut(WheelEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        ::emscripten_sys::emscripten_set_wheel_callback(CString::new("#canvas").unwrap().as_ptr(),
                                                        Box::into_raw(callback) as *mut _,
                                                        1,
                                                        Some(wrapper::<F>));
    }
    unsafe extern "C" fn wrapper<F>(_: c_int,
                                    event: *const ::emscripten_sys::EmscriptenWheelEvent,
                                    arg: *mut c_void)
                                    -> c_int
        where F: FnMut(WheelEvent)
    {
        let event = *event;
        let mut callback = Box::<Box<F>>::from_raw(arg as *mut _);
        let (canvas_x, canvas_y) = into_canvas_pos(event.mouse.canvasX, event.mouse.canvasY);
        callback(WheelEvent {
                     canvas_x,
                     canvas_y,
                     delta: event.deltaY as f64 *
                            match event.deltaMode {
                                0x00 => 1.0,
                                0x01 => 16.0,
                                0x02 => 800.0,
                                _ => panic!("Unexpected event.deltaMode"),
                            },
                 });
        std::mem::forget(callback);
        1
    }
}