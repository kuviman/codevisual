#[cfg(target_os = "emscripten")]
pub trait Setting: ::brijs::IntoJson {}

#[cfg(not(target_os = "emscripten"))]
pub trait Setting {}

pub struct BoolSetting<F: FnMut(bool) + 'static> {
    pub name: String,
    pub default_value: bool,
    pub setter: F,
}

impl<F: FnMut(bool)> Setting for BoolSetting<F> {}

pub struct I32Setting<F: FnMut(i32) + 'static> {
    pub name: String,
    pub default_value: i32,
    pub min_value: i32,
    pub max_value: i32,
    pub setter: F,
}

impl<F: FnMut(i32)> Setting for I32Setting<F> {}

#[cfg(target_os = "emscripten")]
mod implementation {
    use super::*;
    use brijs::IntoJson;

    impl<F: FnMut(bool)> IntoJson for BoolSetting<F> {
        fn into_json(mut self) -> String {
            format!("new CodeVisual.BooleanSetting({}, {}, {})",
                    self.name.into_json(),
                    self.default_value.into_json(),
                    ::brijs::Callback::new(move |value| (self.setter)(value)).into_json())
        }
    }

    impl<F: FnMut(i32)> IntoJson for I32Setting<F> {
        fn into_json(mut self) -> String {
            format!("new CodeVisual.NumberSetting({}, {}, {}, {}, 1, {})",
                    self.name.into_json(),
                    self.min_value.into_json(),
                    self.max_value.into_json(),
                    self.default_value.into_json(),
                    ::brijs::Callback::new(move |value| (self.setter)(value)).into_json())
        }
    }
}

impl ::Application {
    pub fn add_setting<S: Setting>(&self, setting: S) {
        #[cfg(target_os = "emscripten")]
        run_js!{
            CodeVisual.settings.add(setting);
        }
    }
}