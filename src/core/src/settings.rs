#![allow(dead_code, unused_variables)]

use ::*;

enum Setting {
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
}

#[cfg(target_os = "emscripten")]
impl brijs::IntoJson for Setting {
    fn into_json(self) -> String {
        match self {
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
        }
    }
}

pub type SettingValue<T> = Rc<Cell<T>>;

impl Application {
    fn add_setting(&self, setting: Setting) {
        #[cfg(target_os = "emscripten")]
            run_js! {
            CodeVisual.settings.add(setting);
        }
    }
    pub fn add_setting_bool(&self, name: &str, default_value: bool) -> SettingValue<bool> {
        let value = Rc::new(Cell::new(default_value));
        {
            let value = value.clone();
            self.add_setting(Setting::Bool {
                name: String::from(name),
                default_value,
                setter: Box::new(move |x| value.set(x)),
            });
        }
        value
    }
    pub fn add_setting_i32(
        &self,
        name: &str,
        min_value: i32,
        max_value: i32,
        default_value: i32,
    ) -> SettingValue<i32> {
        let value = Rc::new(Cell::new(default_value));
        {
            let value = value.clone();
            self.add_setting(Setting::I32 {
                name: String::from(name),
                default_value,
                min_value,
                max_value,
                setter: Box::new(move |x| value.set(x)),
            });
        }
        value
    }
    pub fn add_setting_f64(
        &self,
        name: &str,
        min_value: f64,
        max_value: f64,
        default_value: f64,
    ) -> SettingValue<f64> {
        let value = Rc::new(Cell::new(default_value));
        {
            let value = value.clone();
            const MAX_INT: i32 = 100500;
            self.add_setting(Setting::I32 {
                name: String::from(name),
                default_value: ((default_value - min_value) / (max_value - min_value) *
                    MAX_INT as f64) as i32,
                min_value: 0,
                max_value: MAX_INT,
                setter: Box::new(move |x| {
                    value.set(
                        min_value + (max_value - min_value) * (x as f64 / MAX_INT as f64),
                    )
                }),
            });
        }
        value
    }
    pub fn add_setting_usize(
        &self,
        name: &str,
        min_value: usize,
        max_value: usize,
        default_value: usize,
    ) -> SettingValue<usize> {
        let value = Rc::new(Cell::new(default_value));
        {
            let value = value.clone();
            self.add_setting(Setting::I32 {
                name: String::from(name),
                default_value: default_value as i32,
                min_value: min_value as i32,
                max_value: max_value as i32,
                setter: Box::new(move |x| value.set(x as usize)),
            });
        }
        value
    }
}
