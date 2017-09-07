pub extern crate prelude;

#[allow(unused_imports)]
#[macro_use]
pub extern crate codevisual_core as core;

pub use core::*;

pub extern crate ugli;

#[cfg(target_os = "emscripten")]
pub extern crate brijs;

#[macro_export]
macro_rules! resources {
    ($name:ident { $($field_name:ident : $field_type:ty = $field_value:tt),*, }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $field_name : $field_type),*
        }
        pub struct Future {
            $(pub $field_name : <$field_type as $crate::Resource>::Future),*
        }
        impl $crate::ResourceFuture<$name> for Future {
            fn unwrap(self) -> $name {
                $name {
                    $($field_name: <$field_type as $crate::Resource>::Future::unwrap(self.$field_name)),*
                }
            }
        }
        impl $crate::Resource for $name {
            type Future = Future;
        }
        macro_rules! load_field {
            ($loader:expr, $field_type2:ty = ()) => {
                <$field_type2 as $crate::ResourceContainer>::load(&$loader)
            };
            ($loader:expr, $field_type2:ty = $path:expr) => {
                <$field_type2 as $crate::Asset>::load(&$loader, $path)
            }
        }
        impl $crate::ResourceContainer for $name {
            fn load(loader: &Rc<$crate::ResourceLoader>) -> Future {
                Future {
                    $($field_name: load_field!(loader, $field_type = $field_value)),*
                }
            }
        }
    }
}

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
