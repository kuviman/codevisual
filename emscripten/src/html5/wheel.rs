use ::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DomDeltaMode {
    Pixel,
    Line,
    Page,
}

impl DomDeltaMode {
    fn from(mode: DOM_DELTA_MODE) -> DomDeltaMode {
        match mode {
            DOM_DELTA_PIXEL => DomDeltaMode::Pixel,
            DOM_DELTA_LINE => DomDeltaMode::Line,
            DOM_DELTA_PAGE => DomDeltaMode::Page,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct WheelEvent {
    pub mouse: MouseEvent,
    pub delta: Vec3<f64>,
    pub delta_mode: DomDeltaMode,
}

impl Deref for WheelEvent {
    type Target = MouseEvent;
    fn deref(&self) -> &MouseEvent {
        &self.mouse
    }
}

impl DerefMut for WheelEvent {
    fn deref_mut(&mut self) -> &mut MouseEvent {
        &mut self.mouse
    }
}

impl WheelEvent {
    fn from(event: &EmscriptenWheelEvent) -> WheelEvent {
        WheelEvent {
            mouse: MouseEvent::from(&event.mouse),
            delta: vec3(event.deltaX as f64, event.deltaY as f64, event.deltaZ as f64),
            delta_mode: DomDeltaMode::from(event.deltaMode),

        }
    }
}

pub fn set_wheel_callback<F>(
    target: Selector,
    use_capture: bool,
    callback: F) -> HtmlResult<()>
    where F: FnMut(WheelEvent) -> bool + 'static {
    let result = unsafe {
        emscripten_set_wheel_callback(selector_as_ptr!(target),
                                      Box::<F>::into_raw(Box::new(callback)) as *mut _,
                                      if use_capture { EM_TRUE } else { EM_FALSE },
                                      Some(wrapper::<F>))
    };
    return match parse_html_result(result) {
        None => Ok(()),
        Some(err) => Err(err),
    };
    #[allow(non_snake_case)]
    unsafe extern "C" fn wrapper<F: FnMut(WheelEvent) -> bool + 'static>(
        eventType: EM_EVENT_TYPE,
        wheelEvent: *const EmscriptenWheelEvent,
        userData: *mut c_void) -> EM_BOOL {
        assert_eq!(eventType, EMSCRIPTEN_EVENT_WHEEL);
        let wheelEvent = &*wheelEvent;
        let mut callback = Box::<F>::from_raw(userData as *mut F);
        let result = callback(WheelEvent::from(wheelEvent));
        mem::forget(callback);
        if result { EM_TRUE } else { EM_FALSE }
    }
}