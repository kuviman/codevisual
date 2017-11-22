use ::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyboardEventType {
    KeyPress,
    KeyDown,
    KeyUp,
}

impl KeyboardEventType {
    fn from(event: EM_EVENT_TYPE) -> KeyboardEventType {
        match event {
            EMSCRIPTEN_EVENT_KEYPRESS => KeyboardEventType::KeyPress,
            EMSCRIPTEN_EVENT_KEYDOWN => KeyboardEventType::KeyDown,
            EMSCRIPTEN_EVENT_KEYUP => KeyboardEventType::KeyUp,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DomKeyLocation {
    Standard,
    Left,
    Right,
    NumPad,
}

impl DomKeyLocation {
    fn from(location: DOM_KEY_LOCATION) -> DomKeyLocation {
        match location {
            DOM_KEY_LOCATION_STANDARD => DomKeyLocation::Standard,
            DOM_KEY_LOCATION_LEFT => DomKeyLocation::Left,
            DOM_KEY_LOCATION_RIGHT => DomKeyLocation::Right,
            DOM_KEY_LOCATION_NUMPAD => DomKeyLocation::NumPad,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct KeyboardEvent<'a> {
    pub key: &'a str,
    pub code: &'a str,
    pub location: DomKeyLocation,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub repeat: bool,
    pub locale: &'a str,
    pub char_value: &'a str,
}

impl<'a> KeyboardEvent<'a> {
    fn from(event: &'a EmscriptenKeyboardEvent) -> KeyboardEvent {
        KeyboardEvent {
            key: unsafe { CStr::from_ptr(event.key.as_ptr()).to_str().unwrap() },
            code: unsafe { CStr::from_ptr(event.code.as_ptr()).to_str().unwrap() },
            location: DomKeyLocation::from(event.location),
            ctrl_key: event.ctrlKey == EM_TRUE,
            shift_key: event.shiftKey == EM_TRUE,
            alt_key: event.altKey == EM_TRUE,
            meta_key: event.metaKey == EM_TRUE,
            repeat: event.repeat == EM_TRUE,
            locale: unsafe { CStr::from_ptr(event.locale.as_ptr()).to_str().unwrap() },
            char_value: unsafe { CStr::from_ptr(event.charValue.as_ptr()).to_str().unwrap() },
        }
    }
}

#[allow(non_snake_case)]
type Registrator = unsafe extern "C" fn(
    target: *const c_char,
    userData: *mut c_void,
    useCapture: EM_BOOL,
    callback: em_key_callback_func) -> EMSCRIPTEN_RESULT;

fn set_callback<F>(
    registrator: Registrator,
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(KeyboardEventType, KeyboardEvent) -> bool + 'static {
    let result = unsafe {
        registrator(selector_as_ptr!(target),
                    Box::<F>::into_raw(Box::new(callback)) as *mut _,
                    if use_capture { EM_TRUE } else { EM_FALSE },
                    Some(wrapper::<F>))
    };
    return match parse_html_result(result) {
        None => Ok(()),
        Some(err) => Err(err),
    };
    #[allow(non_snake_case)]
    unsafe extern "C" fn wrapper<F: FnMut(KeyboardEventType, KeyboardEvent) -> bool + 'static>(
        eventType: EM_EVENT_TYPE,
        keyEvent: *const EmscriptenKeyboardEvent,
        userData: *mut c_void) -> EM_BOOL {
        let keyEvent = &*keyEvent;
        let mut callback = Box::<F>::from_raw(userData as *mut F);
        let result = callback(KeyboardEventType::from(eventType), KeyboardEvent::from(keyEvent));
        mem::forget(callback);
        if result { EM_TRUE } else { EM_FALSE }
    }
}

pub fn set_key_press_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(KeyboardEventType, KeyboardEvent) -> bool + 'static {
    set_callback(emscripten_set_keypress_callback, target, use_capture, callback)
}

pub fn set_key_down_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(KeyboardEventType, KeyboardEvent) -> bool + 'static {
    set_callback(emscripten_set_keydown_callback, target, use_capture, callback)
}

pub fn set_key_up_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(KeyboardEventType, KeyboardEvent) -> bool + 'static {
    set_callback(emscripten_set_keyup_callback, target, use_capture, callback)
}