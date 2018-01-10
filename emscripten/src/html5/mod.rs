use ::*;

macro_rules! selector_as_ptr {
    ($x:expr) => {
        match $x.as_c_str() {
            None => std::ptr::null(),
            Some(s) => s.as_ptr(),
        }
    };
}

mod keys;
mod mouse;
mod wheel;
mod touch;
pub mod webgl;
mod css;

pub use self::keys::*;
pub use self::mouse::*;
pub use self::wheel::*;
pub use self::touch::*;
pub use self::css::*;

#[derive(Debug)]
pub enum HtmlError {
    Deferred,
    NotSupported,
    FailedNotDeferred,
    InvalidTarget,
    UnknownTarget,
    InvalidParam,
    Failed,
    NoData,
    TimedOut,
    Unknown { code: c_int },
}

pub type HtmlResult<T> = Result<T, HtmlError>;

pub(crate) fn parse_html_result(code: EMSCRIPTEN_RESULT) -> Option<HtmlError> {
    match code {
        EMSCRIPTEN_RESULT_SUCCESS => None,
        EMSCRIPTEN_RESULT_DEFERRED => Some(HtmlError::Deferred),
        EMSCRIPTEN_RESULT_NOT_SUPPORTED => Some(HtmlError::NotSupported),
        EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED => Some(HtmlError::FailedNotDeferred),
        EMSCRIPTEN_RESULT_INVALID_TARGET => Some(HtmlError::InvalidTarget),
        EMSCRIPTEN_RESULT_UNKNOWN_TARGET => Some(HtmlError::UnknownTarget),
        EMSCRIPTEN_RESULT_INVALID_PARAM => Some(HtmlError::InvalidParam),
        EMSCRIPTEN_RESULT_FAILED => Some(HtmlError::Failed),
        EMSCRIPTEN_RESULT_NO_DATA => Some(HtmlError::NoData),
        EMSCRIPTEN_RESULT_TIMED_OUT => Some(HtmlError::TimedOut),
        _ => Some(HtmlError::Unknown { code }),
    }
}

#[derive(Debug)]
pub enum Selector<'a> {
    Default,
    Window,
    Document,
    Screen,
    Canvas,
    Element { id: &'a str },
}

impl<'a> Selector<'a> {
    pub(crate) fn as_c_str(&self) -> Option<Cow<CStr>> {
        fn static_selector(selector: &'static [u8]) -> Option<Cow<CStr>> {
            unsafe { Some(Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(selector))) }
        }
        match *self {
            Selector::Default => None,
            Selector::Window => static_selector(b"#window\0"),
            Selector::Document => static_selector(b"#document\0"),
            Selector::Screen => static_selector(b"#screen\0"),
            Selector::Canvas => static_selector(b"#canvas\0"),
            Selector::Element { id } => Some(Cow::Owned(CString::new(id).unwrap())),
        }
    }
}
