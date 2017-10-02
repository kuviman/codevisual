#![deny(warnings)]

pub extern crate prelude;

#[allow(unused_imports)]
#[macro_use]
pub extern crate codevisual_core as core;

pub use core::*;

pub extern crate ugli;

#[cfg(target_os = "emscripten")]
pub extern crate brijs;

#[macro_export]
macro_rules! uniforms {
    () => {
        ()
    };
    ($name:ident : $value:expr) => {
        $crate::ugli::SingleUniform::new(stringify!($name), $value)
    };
    ($name:ident : $value:expr, $($names:ident : $values:expr),+) => {
        (uniforms!($name : $value), uniforms!($($names : $values),+))
    };
    ($($name:ident : $value:expr),*,) => {
        uniforms!($($name : $value),*)
    }
}

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
