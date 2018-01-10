use ::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TouchEventType {
    TouchStart,
    TouchEnd,
    TouchMove,
    TouchCancel,
}

impl TouchEventType {
    fn from(event: EM_EVENT_TYPE) -> TouchEventType {
        match event {
            EMSCRIPTEN_EVENT_TOUCHSTART => TouchEventType::TouchStart,
            EMSCRIPTEN_EVENT_TOUCHEND => TouchEventType::TouchEnd,
            EMSCRIPTEN_EVENT_TOUCHMOVE => TouchEventType::TouchMove,
            EMSCRIPTEN_EVENT_TOUCHCANCEL => TouchEventType::TouchCancel,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TouchPointIdentifier(c_long);

#[derive(Debug, Copy, Clone)]
pub struct TouchPoint {
    pub identifier: TouchPointIdentifier,
    pub screen_pos: Vec2<i32>,
    pub client_pos: Vec2<i32>,
    pub page_pos: Vec2<i32>,
    pub is_changed: bool,
    pub on_target: bool,
    pub target_pos: Vec2<i32>,
    pub canvas_pos: Vec2<i32>,
}

impl TouchPoint {
    fn from(touch: &EmscriptenTouchPoint) -> TouchPoint {
        TouchPoint {
            identifier: TouchPointIdentifier(touch.identifier),
            screen_pos: vec2(touch.screenX as i32, touch.screenY as i32),
            client_pos: vec2(touch.clientX as i32, touch.clientY as i32),
            page_pos: vec2(touch.pageX as i32, touch.pageY as i32),
            is_changed: touch.isChanged != EM_FALSE,
            on_target: touch.onTarget != EM_FALSE,
            target_pos: vec2(touch.targetX as i32, touch.targetY as i32),
            canvas_pos: vec2(touch.canvasX as i32, touch.canvasY as i32),
        }
    }
}

#[derive(Debug)]
pub struct TouchEvent {
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub touches: Vec<TouchPoint>,
}

impl TouchEvent {
    fn from(event: &EmscriptenTouchEvent) -> TouchEvent {
        TouchEvent {
            ctrl_key: event.ctrlKey != EM_FALSE,
            shift_key: event.shiftKey != EM_FALSE,
            alt_key: event.altKey != EM_FALSE,
            meta_key: event.metaKey != EM_FALSE,
            touches: event.touches[..event.numTouches as usize]
                .iter()
                .map(|touch| TouchPoint::from(touch))
                .collect(),
        }
    }
}

#[allow(non_snake_case)]
type Registrator = unsafe extern "C" fn(
    target: *const c_char,
    userData: *mut c_void,
    useCapture: EM_BOOL,
    callback: em_touch_callback_func,
) -> EMSCRIPTEN_RESULT;

fn set_callback<F>(
    registrator: Registrator,
    target: Selector,
    use_capture: bool,
    callback: F,
) -> HtmlResult<()>
where
    F: FnMut(TouchEventType, TouchEvent) -> bool + 'static,
{
    let result = unsafe {
        registrator(
            selector_as_ptr!(target),
            Box::<F>::into_raw(Box::new(callback)) as *mut _,
            if use_capture { EM_TRUE } else { EM_FALSE },
            Some(wrapper::<F>),
        )
    };
    return match parse_html_result(result) {
        None => Ok(()),
        Some(err) => Err(err),
    };
    #[allow(non_snake_case)]
    unsafe extern "C" fn wrapper<F: FnMut(TouchEventType, TouchEvent) -> bool + 'static>(
        eventType: EM_EVENT_TYPE,
        touchEvent: *const EmscriptenTouchEvent,
        userData: *mut c_void,
    ) -> EM_BOOL {
        let touchEvent = &*touchEvent;
        let mut callback = Box::<F>::from_raw(userData as *mut F);
        let result = callback(
            TouchEventType::from(eventType),
            TouchEvent::from(touchEvent),
        );
        mem::forget(callback);
        if result {
            EM_TRUE
        } else {
            EM_FALSE
        }
    }
}

pub fn set_touch_start_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F,
) -> HtmlResult<()>
where
    F: FnMut(TouchEventType, TouchEvent) -> bool + 'static,
{
    set_callback(
        emscripten_set_touchstart_callback,
        target,
        use_capture,
        callback,
    )
}

pub fn set_touch_end_callback<F>(target: Selector, use_capture: bool, callback: F) -> HtmlResult<()>
where
    F: FnMut(TouchEventType, TouchEvent) -> bool + 'static,
{
    set_callback(
        emscripten_set_touchend_callback,
        target,
        use_capture,
        callback,
    )
}

pub fn set_touch_move_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F,
) -> HtmlResult<()>
where
    F: FnMut(TouchEventType, TouchEvent) -> bool + 'static,
{
    set_callback(
        emscripten_set_touchmove_callback,
        target,
        use_capture,
        callback,
    )
}

pub fn set_touch_cancel_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F,
) -> HtmlResult<()>
where
    F: FnMut(TouchEventType, TouchEvent) -> bool + 'static,
{
    set_callback(
        emscripten_set_touchcancel_callback,
        target,
        use_capture,
        callback,
    )
}
