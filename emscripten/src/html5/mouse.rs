use ::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseEventType {
    Click,
    MouseDown,
    MouseUp,
    DblClick,
    MouseMove,
    MouseEnter,
    MouseLeave,
}

impl MouseEventType {
    fn from(event: EM_EVENT_TYPE) -> MouseEventType {
        match event {
            EMSCRIPTEN_EVENT_CLICK => MouseEventType::Click,
            EMSCRIPTEN_EVENT_MOUSEDOWN => MouseEventType::MouseDown,
            EMSCRIPTEN_EVENT_MOUSEUP => MouseEventType::MouseUp,
            EMSCRIPTEN_EVENT_DBLCLICK => MouseEventType::DblClick,
            EMSCRIPTEN_EVENT_MOUSEMOVE => MouseEventType::MouseMove,
            EMSCRIPTEN_EVENT_MOUSEENTER => MouseEventType::MouseEnter,
            EMSCRIPTEN_EVENT_MOUSELEAVE => MouseEventType::MouseLeave,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl MouseButton {
    fn from(button: c_ushort) -> MouseButton {
        match button {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct MouseEvent {
    pub screen_pos: Vec2<i32>,
    pub client_pos: Vec2<i32>,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub button: MouseButton,
    pub movement: Vec2<i32>,
    pub target_pos: Vec2<i32>,
    pub canvas_pos: Vec2<i32>,
    pub padding: i32,
}

impl MouseEvent {
    pub(crate) fn from(event: &EmscriptenMouseEvent) -> MouseEvent {
        MouseEvent {
            screen_pos: vec2(event.screenX as i32, event.screenY as i32),
            client_pos: vec2(event.clientX as i32, event.clientY as i32),
            ctrl_key: event.ctrlKey == EM_TRUE,
            shift_key: event.shiftKey == EM_TRUE,
            alt_key: event.altKey == EM_TRUE,
            meta_key: event.metaKey == EM_TRUE,
            button: MouseButton::from(event.button),
            movement: vec2(event.movementX as i32, event.movementY as i32),
            target_pos: vec2(event.targetX as i32, event.targetY as i32),
            canvas_pos: vec2(event.canvasX as i32, event.canvasY as i32),
            padding: event.padding as i32,
        }
    }
}

#[allow(non_snake_case)]
type Registrator = unsafe extern "C" fn(
    target: *const c_char,
    userData: *mut c_void,
    useCapture: EM_BOOL,
    callback: em_mouse_callback_func) -> EMSCRIPTEN_RESULT;

fn set_callback<F>(
    registrator: Registrator,
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(MouseEventType, MouseEvent) -> bool + 'static {
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
    unsafe extern "C" fn wrapper<F: FnMut(MouseEventType, MouseEvent) -> bool + 'static>(
        eventType: EM_EVENT_TYPE,
        mouseEvent: *const EmscriptenMouseEvent,
        userData: *mut c_void) -> EM_BOOL {
        let mouseEvent = &*mouseEvent;
        let mut callback = Box::<F>::from_raw(userData as *mut F);
        let result = callback(MouseEventType::from(eventType), MouseEvent::from(mouseEvent));
        mem::forget(callback);
        if result { EM_TRUE } else { EM_FALSE }
    }
}

pub fn set_click_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(MouseEventType, MouseEvent) -> bool + 'static {
    set_callback(emscripten_set_click_callback, target, use_capture, callback)
}

pub fn set_mouse_down_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(MouseEventType, MouseEvent) -> bool + 'static {
    set_callback(emscripten_set_mousedown_callback, target, use_capture, callback)
}

pub fn set_mouse_up_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(MouseEventType, MouseEvent) -> bool + 'static {
    set_callback(emscripten_set_mouseup_callback, target, use_capture, callback)
}

pub fn set_dbl_click_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(MouseEventType, MouseEvent) -> bool + 'static {
    set_callback(emscripten_set_dblclick_callback, target, use_capture, callback)
}

pub fn set_mouse_move_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(MouseEventType, MouseEvent) -> bool + 'static {
    set_callback(emscripten_set_mousemove_callback, target, use_capture, callback)
}

pub fn set_mouse_enter_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(MouseEventType, MouseEvent) -> bool + 'static {
    set_callback(emscripten_set_mouseenter_callback, target, use_capture, callback)
}

pub fn set_mouse_leave_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(MouseEventType, MouseEvent) -> bool + 'static {
    set_callback(emscripten_set_mouseleave_callback, target, use_capture, callback)
}