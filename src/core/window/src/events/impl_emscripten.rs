use ::*;

impl From<brijs::MouseButton> for MouseButton {
    fn from(button: brijs::MouseButton) -> Self {
        use brijs::MouseButton as BMB;
        match button {
            BMB::Left => MouseButton::Left,
            BMB::Middle => MouseButton::Middle,
            BMB::Right => MouseButton::Right,
        }
    }
}

impl From<brijs::MouseDownEvent> for Event {
    fn from(event: brijs::MouseDownEvent) -> Self {
        Event::MouseDown {
            position: event.canvas_pos,
            button: event.button.into(),
        }
    }
}

impl From<brijs::MouseUpEvent> for Event {
    fn from(event: brijs::MouseUpEvent) -> Self {
        Event::MouseUp {
            position: event.canvas_pos,
            button: event.button.into(),
        }
    }
}

impl From<brijs::MouseMoveEvent> for Event {
    fn from(event: brijs::MouseMoveEvent) -> Self {
        Event::MouseMove { position: event.canvas_pos }
    }
}

impl From<brijs::WheelEvent> for Event {
    fn from(event: brijs::WheelEvent) -> Self {
        Event::Wheel { delta: event.delta }
    }
}

impl From<brijs::TouchPoint> for TouchPoint {
    fn from(point: brijs::TouchPoint) -> Self {
        TouchPoint { position: point.canvas_pos }
    }
}

fn convert_touches(touches: Vec<brijs::TouchPoint>) -> Vec<TouchPoint> {
    touches.into_iter().map(|touch| touch.into()).collect()
}

impl From<brijs::TouchStartEvent> for Event {
    fn from(event: brijs::TouchStartEvent) -> Self {
        Event::TouchStart { touches: convert_touches(event.touches) }
    }
}

impl From<brijs::TouchEndEvent> for Event {
    fn from(event: brijs::TouchEndEvent) -> Self {
        Event::TouchEnd
    }
}

impl From<brijs::TouchMoveEvent> for Event {
    fn from(event: brijs::TouchMoveEvent) -> Self {
        Event::TouchMove { touches: convert_touches(event.touches) }
    }
}

impl Window {
    pub fn get_events(&self) -> Vec<Event> {
        lazy_static! {
            static ref EVENTS: Arc<Mutex<Vec<Event>>> = {
                let events = Arc::new(Mutex::new(Vec::new()));
                macro_rules! setup {
                    ($setter:ident, $events:ident) => {
                        let events = $events.clone();
                        brijs::$setter(move |event| events.lock().unwrap().push(event.into()));
                    }
                }
                setup!(set_mousedown_callback, events);
                setup!(set_mouseup_callback, events);
                setup!(set_mousemove_callback, events);
                setup!(set_touchstart_callback, events);
                setup!(set_touchend_callback, events);
                setup!(set_touchmove_callback, events);
                setup!(set_wheel_callback, events);
                events
            };
        }
        EVENTS.lock().unwrap().split_off(0)
    }
}