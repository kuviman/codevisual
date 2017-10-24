use ::*;

fn convert_key(code: String) -> Key {
    use Key::*;
    match code.as_str() {
        "KeyA" => A,
        "KeyB" => B,
        "KeyC" => C,
        "KeyD" => D,
        "KeyE" => E,
        "KeyF" => F,
        "KeyG" => G,
        "KeyH" => H,
        "KeyI" => I,
        "KeyJ" => J,
        "KeyK" => K,
        "KeyL" => L,
        "KeyM" => M,
        "KeyN" => N,
        "KeyO" => O,
        "KeyP" => P,
        "KeyQ" => Q,
        "KeyR" => R,
        "KeyS" => S,
        "KeyT" => T,
        "KeyU" => U,
        "KeyV" => V,
        "KeyW" => W,
        "KeyX" => X,
        "KeyY" => Y,
        "KeyZ" => Z,

        "Digit0" => Num0,
        "Digit1" => Num1,
        "Digit2" => Num2,
        "Digit3" => Num3,
        "Digit4" => Num4,
        "Digit5" => Num5,
        "Digit6" => Num6,
        "Digit7" => Num7,
        "Digit8" => Num8,
        "Digit9" => Num9,

        "Escape" => Escape,
        "Space" => Space,

        "ShiftLeft" => LShift,
        "ShiftRight" => RShift,

        "ControlLeft" => LCtrl,
        "ControlRight" => RCtrl,

        "AltLeft" => LAlt,
        "AltRight" => RAlt,

        _ => {
            eprintln!("Key unrecognized: {:?}", code);
            Key::Unknown
        }
    }
}

impl From<web::KeyDownEvent> for Event {
    fn from(event: web::KeyDownEvent) -> Self {
        Event::KeyDown {
            key: convert_key(event.key.code),
        }
    }
}

impl From<web::KeyUpEvent> for Event {
    fn from(event: web::KeyUpEvent) -> Self {
        Event::KeyUp {
            key: convert_key(event.key.code),
        }
    }
}

impl From<web::MouseButton> for MouseButton {
    fn from(button: web::MouseButton) -> Self {
        use web::MouseButton as BMB;
        match button {
            BMB::Left => MouseButton::Left,
            BMB::Middle => MouseButton::Middle,
            BMB::Right => MouseButton::Right,
        }
    }
}

impl From<web::MouseDownEvent> for Event {
    fn from(event: web::MouseDownEvent) -> Self {
        Event::MouseDown {
            position: event.canvas_pos,
            button: event.button.into(),
        }
    }
}

impl From<web::MouseUpEvent> for Event {
    fn from(event: web::MouseUpEvent) -> Self {
        Event::MouseUp {
            position: event.canvas_pos,
            button: event.button.into(),
        }
    }
}

impl From<web::MouseMoveEvent> for Event {
    fn from(event: web::MouseMoveEvent) -> Self {
        Event::MouseMove { position: event.canvas_pos }
    }
}

impl From<web::WheelEvent> for Event {
    fn from(event: web::WheelEvent) -> Self {
        Event::Wheel { delta: event.delta }
    }
}

impl From<web::TouchPoint> for TouchPoint {
    fn from(point: web::TouchPoint) -> Self {
        TouchPoint { position: point.canvas_pos }
    }
}

fn convert_touches(touches: Vec<web::TouchPoint>) -> Vec<TouchPoint> {
    touches.into_iter().map(|touch| touch.into()).collect()
}

impl From<web::TouchStartEvent> for Event {
    fn from(event: web::TouchStartEvent) -> Self {
        Event::TouchStart { touches: convert_touches(event.touches) }
    }
}

impl From<web::TouchEndEvent> for Event {
    fn from(_: web::TouchEndEvent) -> Self {
        Event::TouchEnd
    }
}

impl From<web::TouchMoveEvent> for Event {
    fn from(event: web::TouchMoveEvent) -> Self {
        Event::TouchMove { touches: convert_touches(event.touches) }
    }
}

impl Window {
    pub ( crate ) fn internal_get_events(&self) -> Vec<Event> {
        lazy_static! {
            static ref EVENTS: Arc<Mutex<Vec<Event>>> = {
                let events = Arc::new(Mutex::new(Vec::new()));
                macro_rules! setup {
                    ($setter:ident, $events:ident) => {
                        let events = $events.clone();
                        web::$setter(move |event| events.lock().unwrap().push(event.into()));
                    }
                }
                setup!(set_mousedown_callback, events);
                setup!(set_mouseup_callback, events);
                setup!(set_mousemove_callback, events);
                setup!(set_touchstart_callback, events);
                setup!(set_touchend_callback, events);
                setup!(set_touchmove_callback, events);
                setup!(set_wheel_callback, events);
                setup!(set_keydown_callback, events);
                setup!(set_keyup_callback, events);
                events
            };
        }
        EVENTS.lock().unwrap().split_off(0)
    }
}
