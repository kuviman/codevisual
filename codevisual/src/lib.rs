#![deny(warnings)]

#[cfg(not(target_os = "emscripten"))]
extern crate image;
#[cfg(not(target_os = "emscripten"))]
extern crate rodio;
#[cfg(not(target_os = "emscripten"))]
extern crate glutin;
extern crate rusttype;
extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[cfg(not(target_os = "emscripten"))]
extern crate threadpool;
#[cfg(not(target_os = "emscripten"))]
extern crate num_cpus;
pub extern crate prelude;
#[macro_use]
extern crate ugli;
#[cfg(target_os = "emscripten")]
#[macro_use]
pub extern crate webby;
#[allow(unused_imports)]
#[macro_use]
extern crate codevisual_derive;

pub(crate) use prelude::*;
pub use codevisual_derive::*;
#[cfg(target_os = "emscripten")]
pub(crate) use webby::emscripten;

#[macro_export]
macro_rules! defines {
    () => {
        &()
    };
    ($name:ident : $value:expr) => {
        &$crate::SingleShaderDefine::new(stringify!($name), $value)
    };
    ($name:ident : $value:expr, $($names:ident : $values:expr),+) => {
        (defines!($name : $ident), defines!($($names : $values),*,))
    };
    ($($name:ident : $value:expr),*,) => {
        defines!($($name : $value),*)
    }
}

#[macro_export]
macro_rules! impl_shader_library {
    ($name:ident { $($path:expr => $value:expr),*, }) => {
        impl $crate::ShaderLibrary for $name {
            fn get(path: &str) -> Option<&str> {
                if let Some(result) = <$crate::ShaderPrelude as $crate::ShaderLibrary>::get(path) {
                    return Some(result);
                }
                match path {
                    $($path => Some($value)),*,
                    _ => None
                }
            }
        }
    }
}

mod app;
mod window;
mod material;
mod resources;
mod settings;
mod sound;
mod font;

pub use self::app::*;
pub use self::material::*;
pub use self::window::*;
pub use self::resources::*;
pub use self::settings::*;
pub use self::sound::*;
pub use self::font::*;