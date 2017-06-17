use std::sync::Mutex;

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub enum Event {
    MouseDown { x: i32, y: i32, button: MouseButton },
    MouseUp { x: i32, y: i32, button: MouseButton },
    MouseMove { x: i32, y: i32 },
    Wheel { delta: f64 },
}

lazy_static!{
    static ref EVENTS: Mutex<Vec<Event>> = Mutex::new(Vec::new());
}

trait EmscriptenEvent {
    fn into_event(self) -> Event;
}

impl EmscriptenEvent for ::emscripten::MouseDownEvent {
    fn into_event(self) -> Event {
        Event::MouseDown {
            x: self.canvas_x,
            y: self.canvas_y,
            button: {
                match self.button {
                    ::emscripten::MouseButton::Left => MouseButton::Left,
                    ::emscripten::MouseButton::Middle => MouseButton::Middle,
                    ::emscripten::MouseButton::Right => MouseButton::Right,
                }
            },
        }
    }
}

impl EmscriptenEvent for ::emscripten::MouseUpEvent {
    fn into_event(self) -> Event {
        Event::MouseUp {
            x: self.canvas_x,
            y: self.canvas_y,
            button: {
                match self.button {
                    ::emscripten::MouseButton::Left => MouseButton::Left,
                    ::emscripten::MouseButton::Middle => MouseButton::Middle,
                    ::emscripten::MouseButton::Right => MouseButton::Right,
                }
            },
        }
    }
}

impl EmscriptenEvent for ::emscripten::MouseMoveEvent {
    fn into_event(self) -> Event {
        Event::MouseMove {
            x: self.canvas_x,
            y: self.canvas_y,
        }
    }
}

impl EmscriptenEvent for ::emscripten::WheelEvent {
    fn into_event(self) -> Event {
        Event::Wheel { delta: self.delta }
    }
}

pub(crate) fn init() {
    ::emscripten::set_mousedown_callback(|event| {
                                             EVENTS.lock().unwrap().push(event.into_event());
                                         });
    ::emscripten::set_mouseup_callback(|event| {
                                           EVENTS.lock().unwrap().push(event.into_event());
                                       });
    ::emscripten::set_mousemove_callback(|event| {
                                             EVENTS.lock().unwrap().push(event.into_event());
                                         });
    ::emscripten::set_wheel_callback(|event| {
                                         EVENTS.lock().unwrap().push(event.into_event());
                                     });
}

pub(crate) struct EventIterator;

impl Iterator for EventIterator {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        EVENTS.lock().unwrap().pop()
    }
}

pub(crate) fn get() -> EventIterator {
    EventIterator
}