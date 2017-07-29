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
}

#[cfg(target_os = "emscripten")]
impl brijs::IntoJson for Setting {
    fn into_json(mut self) -> String {
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

impl Application {
    pub fn add_setting(&self, setting: Setting) {
        #[cfg(target_os = "emscripten")]
        run_js!{
            CodeVisual.settings.add(setting);
        }
    }
}
