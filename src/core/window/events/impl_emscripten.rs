use ::*;

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
                events
            };
        }
        EVENTS.lock().unwrap().split_off(0)
    }
}
