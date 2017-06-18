use std;

pub enum Setting<'a> {
    Bool {
        name: &'a str,
        default_value: bool,
        setter: &'a mut FnMut(bool),
    },
    I32 {
        name: &'a str,
        min_value: i32,
        max_value: i32,
        default_value: i32,
        setter: &'a mut FnMut(i32),
    },
}

struct BoolFunc<'a>(&'a mut FnMut(bool));
struct I32Func<'a>(&'a mut FnMut(i32));

use emscripten::IntoJson;

impl<'a> IntoJson for BoolFunc<'a> {
    fn into_json(self) -> String {
        let boxed = Box::new(self);
        extern "C" fn wrapper(f: std::os::raw::c_int, b: std::os::raw::c_int) {
            let boxed = unsafe { Box::from_raw(f as *mut BoolFunc) };
            boxed.0(b != 0);
            std::mem::forget(boxed);
        }
        let result = format!("function(b) {{ Runtime.dynCall('vii', {}, [{}, b ? 1 : 0]); }}",
                             (wrapper as std::os::raw::c_int).into_json(),
                             (Box::into_raw(boxed) as std::os::raw::c_int).into_json());
        result
    }
}

impl<'a> IntoJson for I32Func<'a> {
    fn into_json(self) -> String {
        let boxed = Box::new(self);
        extern "C" fn wrapper(f: std::os::raw::c_int, b: std::os::raw::c_int) {
            let boxed = unsafe { Box::from_raw(f as *mut I32Func) };
            boxed.0(b as i32);
            std::mem::forget(boxed);
        }
        let result = format!("function(i) {{ Runtime.dynCall('vii', {}, [{}, i]); }}",
                             (wrapper as std::os::raw::c_int).into_json(),
                             (Box::into_raw(boxed) as std::os::raw::c_int).into_json());
        result
    }
}

impl<'a> IntoJson for Setting<'a> {
    fn into_json(self) -> String {
        match self {
            Setting::Bool {
                name,
                default_value,
                setter,
            } => {
                format!("new CodeVisual.BooleanSetting({}, {}, {})",
                        name.into_json(),
                        default_value.into_json(),
                        BoolFunc(setter).into_json())
            }
            Setting::I32 {
                name,
                min_value,
                max_value,
                default_value,
                setter,
            } => {
                format!("new CodeVisual.NumberSetting({}, {}, {}, {}, 1, {})",
                        name.into_json(),
                        min_value.into_json(),
                        max_value.into_json(),
                        default_value.into_json(),
                        I32Func(setter).into_json())
            }
        }
    }
}

impl ::Application {
    pub fn add_setting(&self, setting: Setting) {
        #[cfg(target_os = "emscripten")]
        run_js!{
            CodeVisual.settings.add(setting);
        }
    }
}