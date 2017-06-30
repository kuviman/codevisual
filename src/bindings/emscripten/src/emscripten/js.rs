use ::*;
use std::marker::PhantomData;

pub fn run_script(script: &str) {
    let script = CString::new(script).expect("Could not convert script to C string");
    unsafe {
        ::emscripten_sys::emscripten_run_script(script.as_ptr());
    }
}

#[macro_export]
macro_rules! format_placeholders {
    () => ("");
    ($arg:expr) => ("{}");
    ($head:expr, $($tail:expr),+) => (
        concat!("{},", format_placeholders!($($tail),+))
    )
}

#[macro_export]
macro_rules! run_js {
    ($($($f:ident).+ ( $($args:expr),* );)*) => (
        $(
            $crate::run_script(&format!(
                concat!(stringify!($($f).+), "(", format_placeholders!($($args),*), ")"),
                $($crate::IntoJson::into_json($args)),*));
        )*
    )
}

pub trait IntoJson {
    fn into_json(self) -> String;
}

impl<'a, T: ?Sized + serde::Serialize> IntoJson for &'a T {
    fn into_json(self) -> String {
        ::serde_json::to_string(self).expect("Could not convert to JSON")
    }
}

pub struct Callback<T, F: FnMut(T) + 'static> {
    f: F,
    phantom_data: PhantomData<T>,
}

impl<T, F: FnMut(T)> Callback<T, F> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            phantom_data: PhantomData,
        }
    }
}

impl<F: FnMut(bool)> IntoJson for Callback<bool, F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: FnMut(bool)>(f: c_int, b: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed(b != 0);
            std::mem::forget(boxed);
        }
        format!("function(b) {{ Runtime.dynCall('vii', {}, [{}, b ? 1 : 0]); }}",
                (wrapper::<F> as c_int).into_json(),
                (Box::into_raw(boxed) as c_int).into_json())
    }
}

impl<F: FnMut(i32)> IntoJson for Callback<i32, F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: FnMut(i32)>(f: c_int, b: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed(b as i32);
            std::mem::forget(boxed);
        }
        format!("function(i) {{ Runtime.dynCall('vii', {}, [{}, i]); }}",
                (wrapper::<F> as c_int).into_json(),
                (Box::into_raw(boxed) as c_int).into_json())
    }
}