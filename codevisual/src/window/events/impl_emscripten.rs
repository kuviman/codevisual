use ::*;

fn convert_key(code: &str) -> Key {
    use Key::*;
    match code {
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
        "Enter" => Enter,
        "Backspace" => Backspace,

        "ShiftLeft" => LShift,
        "ShiftRight" => RShift,

        "ControlLeft" => LCtrl,
        "ControlRight" => RCtrl,

        "AltLeft" => LAlt,
        "AltRight" => RAlt,

        "ArrowLeft" => Left,
        "ArrowRight" => Right,
        "ArrowUp" => Up,
        "ArrowDown" => Down,

        "PageUp" => PageUp,
        "PageDown" => PageDown,

        _ => {
            //            eprintln!("Key unrecognized: {:?}", code);
            Key::Unknown
        }
    }
}

impl<'a> From<(emscripten::KeyboardEventType, emscripten::KeyboardEvent<'a>)> for Event {
    fn from((typ, event): (emscripten::KeyboardEventType, emscripten::KeyboardEvent<'a>)) -> Event {
        match typ {
            emscripten::KeyboardEventType::KeyDown => Event::KeyDown {
                key: convert_key(event.code),
            },
            emscripten::KeyboardEventType::KeyUp => Event::KeyUp {
                key: convert_key(event.code),
            },
            _ => unimplemented!()
        }
    }
}

impl From<emscripten::MouseButton> for MouseButton {
    fn from(button: emscripten::MouseButton) -> Self {
        use emscripten::MouseButton as EMB;
        match button {
            EMB::Left => MouseButton::Left,
            EMB::Middle => MouseButton::Middle,
            EMB::Right => MouseButton::Right,
        }
    }
}

impl From<(emscripten::MouseEventType, emscripten::MouseEvent)> for Event {
    fn from((typ, event): (emscripten::MouseEventType, emscripten::MouseEvent)) -> Event {
        match typ {
            emscripten::MouseEventType::MouseDown => Event::MouseDown {
                position: vec2(event.canvas_pos.x as f64, event.canvas_pos.y as f64),
                button: event.button.into(),
            },
            emscripten::MouseEventType::MouseUp => Event::MouseUp {
                position: vec2(event.canvas_pos.x as f64, event.canvas_pos.y as f64),
                button: event.button.into(),
            },
            emscripten::MouseEventType::MouseMove => Event::MouseMove {
                position: vec2(event.canvas_pos.x as f64, event.canvas_pos.y as f64),
            },
            _ => unimplemented!()
        }
    }
}

impl From<emscripten::WheelEvent> for Event {
    fn from(event: emscripten::WheelEvent) -> Event {
        Event::Wheel {
            delta: event.delta.y as f64 * match event.delta_mode {
                emscripten::DomDeltaMode::Pixel => 1.0,
                emscripten::DomDeltaMode::Line => 17.0,
                emscripten::DomDeltaMode::Page => 800.0,
            },
        }
    }
}

impl From<emscripten::TouchPoint> for TouchPoint {
    fn from(touch: emscripten::TouchPoint) -> TouchPoint {
        TouchPoint {
            position: vec2(touch.canvas_pos.x as f64, touch.canvas_pos.y as f64),
        }
    }
}

impl From<(emscripten::TouchEventType, emscripten::TouchEvent)> for Event {
    fn from((typ, event): (emscripten::TouchEventType, emscripten::TouchEvent)) -> Event {
        match typ {
            emscripten::TouchEventType::TouchStart => Event::TouchStart {
                touches: event.touches.into_iter().map(TouchPoint::from).collect(),
            },
            emscripten::TouchEventType::TouchEnd => Event::TouchEnd,
            emscripten::TouchEventType::TouchMove => Event::TouchMove {
                touches: event.touches.into_iter().map(TouchPoint::from).collect(),
            },
            _ => unimplemented!()
        }
    }
}

fn init_events() -> emscripten::HtmlResult<Arc<Mutex<Vec<Event>>>> {
    let events = Arc::new(Mutex::new(Vec::new()));
    emscripten::set_mouse_down_callback(emscripten::Selector::Canvas, true, {
        let events = events.clone();
        move |typ, event| {
            events.lock().unwrap().push((typ, event).into());
            true
        }
    })?;
    emscripten::set_mouse_up_callback(emscripten::Selector::Canvas, true, {
        let events = events.clone();
        move |typ, event| {
            events.lock().unwrap().push((typ, event).into());
            true
        }
    })?;
    emscripten::set_mouse_move_callback(emscripten::Selector::Canvas, true, {
        let events = events.clone();
        move |typ, event| {
            events.lock().unwrap().push((typ, event).into());
            true
        }
    })?;
    emscripten::set_touch_start_callback(emscripten::Selector::Canvas, true, {
        let events = events.clone();
        move |typ, event| {
            events.lock().unwrap().push((typ, event).into());
            true
        }
    })?;
    emscripten::set_touch_end_callback(emscripten::Selector::Canvas, true, {
        let events = events.clone();
        move |typ, event| {
            events.lock().unwrap().push((typ, event).into());
            true
        }
    })?;
    emscripten::set_touch_move_callback(emscripten::Selector::Canvas, true, {
        let events = events.clone();
        move |typ, event| {
            events.lock().unwrap().push((typ, event).into());
            true
        }
    })?;
    emscripten::set_key_down_callback(emscripten::Selector::Canvas, true, {
        let events = events.clone();
        move |typ, event| {
            events.lock().unwrap().push((typ, event).into());
            true
        }
    })?;
    emscripten::set_key_up_callback(emscripten::Selector::Canvas, true, {
        let events = events.clone();
        move |typ, event| {
            events.lock().unwrap().push((typ, event).into());
            true
        }
    })?;
    emscripten::set_wheel_callback(emscripten::Selector::Canvas, true, {
        let events = events.clone();
        move |event| {
            events.lock().unwrap().push(event.into());
            true
        }
    })?;
    Ok(events)
}

impl Window {
    pub(crate) fn internal_get_events(&self) -> Vec<Event> {
        lazy_static! {
            static ref EVENTS: Arc < Mutex < Vec <Event > > > = init_events().unwrap();
        }
        EVENTS.lock().unwrap().split_off(0)
    }
}
