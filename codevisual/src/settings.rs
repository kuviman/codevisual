#![allow(dead_code, unused_variables)]

use ::*;

pub enum Setting {
    Bool {
        name: String,
        default: bool,
        setter: Box<FnMut(bool)>,
    },
    I32 {
        name: String,
        default: i32,
        range: Range<i32>,
        setter: Box<FnMut(i32)>,
    },
}

impl Setting {
    pub fn create_range<T: Num + num::NumCast + Copy + 'static, S: FnMut(T) + 'static>(
        name: &str,
        default: T,
        range: Range<T>,
        mut setter: S,
    ) -> Setting {
        // TODO: check for float/int better?
        if T::one() / (T::one() + T::one()) == T::zero() {
            Setting::I32 {
                name: String::from(name),
                default: default.to_i32().unwrap(),
                range: range.start.to_i32().unwrap()..range.end.to_i32().unwrap() - 1,
                setter: Box::new(move |val| {
                    setter(T::from(val).unwrap());
                }),
            }
        } else {
            const STEPS: i32 = 100500;
            Setting::I32 {
                name: String::from(name),
                default: (T::from(STEPS).unwrap() * (default - range.start)
                    / (range.end - range.start))
                    .to_i32()
                    .unwrap(),
                range: 0..STEPS + 1,
                setter: Box::new(move |val| {
                    setter(
                        T::from(val).unwrap() / T::from(STEPS).unwrap() * (range.end - range.start)
                            + range.start,
                    );
                }),
            }
        }
    }
}

#[cfg(target_os = "emscripten")]
impl webby::IntoJson for Setting {
    fn into_json(self) -> String {
        match self {
            Setting::Bool {
                name,
                default,
                mut setter,
            } => format!(
                "new CodeVisual.BooleanSetting({}, {}, {})",
                name.into_json(),
                default.into_json(),
                webby::Callback::from(move |value| setter(value)).into_json()
            ),
            Setting::I32 {
                name,
                default,
                range,
                mut setter,
            } => format!(
                "new CodeVisual.NumberSetting({}, {}, {}, {}, 1, {})",
                name.into_json(),
                range.start.into_json(),
                range.end.into_json(),
                default.into_json(),
                webby::Callback::from(move |value| setter(value)).into_json()
            ),
        }
    }
}

pub trait Settings {
    fn register(app: &App) -> Rc<RefCell<Self>>;
}

impl App {
    pub fn add_setting(&self, setting: Setting) {
        #[cfg(target_os = "emscripten")]
        js! {
            CodeVisual.settings.add(@(setting));
        }
    }
    pub fn register_settings<S: Settings>(&self) -> Rc<RefCell<S>> {
        S::register(self)
    }
}
