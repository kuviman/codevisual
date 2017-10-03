#![allow(dead_code, unused_variables)]

use ::*;

pub enum Setting {
    Bool {
        name: String,
        default_value: bool,
        setter: Box<FnMut(bool)>,
    },
    I32 {
        name: String,
        default_value: i32,
        min_value: i32,
        max_value: i32,
        setter: Box<FnMut(i32)>,
    },
    F32 {
        name: String,
        default_value: f32,
        min_value: f32,
        max_value: f32,
        setter: Box<FnMut(f32)>,
    },
    F64 {
        name: String,
        default_value: f64,
        min_value: f64,
        max_value: f64,
        setter: Box<FnMut(f64)>,
    },
    Usize {
        name: String,
        default_value: usize,
        min_value: usize,
        max_value: usize,
        setter: Box<FnMut(usize)>,
    }
}

#[cfg(target_os = "emscripten")]
impl brijs::IntoJson for Setting {
    fn into_json(self) -> String {
        const MAX_INT: i32 = 100500;
        let setting = match self {
            Setting::Bool { .. } => self,
            Setting::I32 { .. } => self,
            Setting::F32 { name, default_value, min_value, max_value, setter } => {
                Setting::I32 {
                    name,
                    default_value: (self.default_value * MAX_INT as f32) as i32,
                    min_value: (self.min_value * MAX_INT as f32) as i32,
                    max_value: (self.max_value * MAX_INT as f32) as i32,
                    setter: Box::new(move |value| {
                        setter(value as f32 / MAX_INT as f32);
                    }),
                }
            }
            Setting::F64 { name, default_value, min_value, max_value, setter } => {
                Setting::I32 {
                    name,
                    default_value: (self.default_value * MAX_INT as f64) as i32,
                    min_value: (self.min_value * MAX_INT as f64) as i32,
                    max_value: (self.max_value * MAX_INT as f64) as i32,
                    setter: Box::new(move |value| {
                        setter(value as f64 / MAX_INT as f64);
                    }),
                }
            }
            Setting::Usize { name, default_value, min_value, max_value, setter } => {
                Setting::I32 {
                    name,
                    default_value: default_value as i32,
                    min_value: min_value as i32,
                    max_value: max_value as i32,
                    setter: Box::new(move |value| {
                        setter(value as usize);
                    }),
                }
            }
        };
        match setting {
            Setting::Bool {
                name,
                default_value,
                mut setter,
            } => {
                format!(
                    "new CodeVisual.BooleanSetting({}, {}, {})",
                    name.into_json(),
                    default_value.into_json(),
                    brijs::Callback::from(move |value| setter(value)).into_json()
                )
            }
            Setting::I32 {
                name,
                default_value,
                min_value,
                max_value,
                mut setter,
            } => {
                format!(
                    "new CodeVisual.NumberSetting({}, {}, {}, {}, 1, {})",
                    name.into_json(),
                    min_value.into_json(),
                    max_value.into_json(),
                    default_value.into_json(),
                    brijs::Callback::from(move |value| setter(value)).into_json()
                )
            }
            _ => panic!("Setting should be converted to bool or i32 setting first")
        }
    }
}

pub trait Settings {
    fn register(app: &Application) -> Rc<RefCell<Self>>;
}

impl Application {
    pub fn add_setting(&self, setting: Setting) {
        #[cfg(target_os = "emscripten")]
            run_js! {
            CodeVisual.settings.add(setting);
        }
    }
    pub fn register_settings<S: Settings>(&self) -> Rc<RefCell<S>> {
        S::register(self)
    }
}
