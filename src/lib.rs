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
macro_rules! shader_library {
    ($name:ident { $($path:expr => $value:expr),*, }) => {
        pub struct ShaderLibrary;
        impl $crate::ShaderLibrary for ShaderLibrary {
            fn get(path: &str) -> Option<&str> {
                match path {
                    $($path => Some($value)),*,
                    _ => None
                }
            }
        }
        pub type $name = ($crate::ShaderPrelude, ShaderLibrary);
    }
}
