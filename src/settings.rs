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

impl<'a> ::IntoJson for BoolFunc<'a> {
    fn into(self) -> String {
        let boxed = Box::new(self);
        extern "C" fn wrapper(f: std::os::raw::c_int, b: std::os::raw::c_int) {
            let boxed = unsafe { Box::from_raw(f as *mut BoolFunc) };
            boxed.0(b != 0);
            std::mem::forget(boxed);
        }
        let result = format!("function(b) {{ Runtime.dynCall('vii', {}, [{}, b ? 1 : 0]); }}",
                             ::IntoJson::into(&(wrapper as std::os::raw::c_int)),
                             ::IntoJson::into(&(Box::into_raw(boxed) as std::os::raw::c_int)));
        result
    }
}

impl<'a> ::IntoJson for I32Func<'a> {
    fn into(self) -> String {
        let boxed = Box::new(self);
        extern "C" fn wrapper(f: std::os::raw::c_int, b: std::os::raw::c_int) {
            let boxed = unsafe { Box::from_raw(f as *mut I32Func) };
            boxed.0(b as i32);
            std::mem::forget(boxed);
        }
        let result = format!("function(i) {{ Runtime.dynCall('vii', {}, [{}, i]); }}",
                             ::IntoJson::into(&(wrapper as std::os::raw::c_int)),
                             ::IntoJson::into(&(Box::into_raw(boxed) as std::os::raw::c_int)));
        result
    }
}

impl<'a> ::IntoJson for Setting<'a> {
    fn into(self) -> String {
        match self {
            Setting::Bool {
                name,
                default_value,
                setter,
            } => {
                format!("new CodeVisual.BooleanSetting({}, {}, {})",
                        ::IntoJson::into(name),
                        ::IntoJson::into(&default_value),
                        ::IntoJson::into(BoolFunc(setter)))
            }
            Setting::I32 {
                name,
                min_value,
                max_value,
                default_value,
                setter,
            } => {
                format!("new CodeVisual.NumberSetting({}, {}, {}, {}, 1, {})",
                        ::IntoJson::into(name),
                        ::IntoJson::into(&min_value),
                        ::IntoJson::into(&max_value),
                        ::IntoJson::into(&default_value),
                        ::IntoJson::into(I32Func(setter)))
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