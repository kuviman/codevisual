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

pub(crate) struct EventIterator;

impl Iterator for EventIterator {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        EVENTS.lock().unwrap().pop()
    }
}

#[cfg(target_os = "emscripten")]
mod implementation {
    use super::*;

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

    pub fn init() {
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

    pub(crate) fn get() -> EventIterator {
        EventIterator
    }
}

#[cfg(not(target_os = "emscripten"))]
mod implementation {
    use super::*;

    pub fn init() {}

    lazy_static!{ static ref SHOULD_CLOSE: Mutex<bool> = Mutex::new(false); }
    lazy_static!{ static ref MOUSE_POS: Mutex<(i32, i32)> = Mutex::new((0, 0)); }

    pub(crate) fn get() -> EventIterator {
        let mut events = EVENTS.lock().unwrap();
        ::Application::get_instance()
            .events_loop
            .poll_events(|e| {
                let ::glutin::Event::WindowEvent { event: e, .. } = e;
                match e {
                    ::glutin::WindowEvent::Closed => *SHOULD_CLOSE.lock().unwrap() = true,
                    ::glutin::WindowEvent::MouseWheel(delta, ..) => {
                        events.push(Event::Wheel {
                                        delta: match delta {
                                            ::glutin::MouseScrollDelta::PixelDelta(_, dy) => {
                                                -dy as f64
                                            }
                                            ::glutin::MouseScrollDelta::LineDelta(_, dy) => {
                                                -dy as f64 * 16.0
                                            }
                                        },
                                    });
                    }
                    ::glutin::WindowEvent::MouseMoved(x, y) => {
                        *MOUSE_POS.lock().unwrap() = (x, y);
                        events.push(Event::MouseMove { x, y })
                    }
                    ::glutin::WindowEvent::MouseInput(state, button) => {
                        let button = match button {
                            ::glutin::MouseButton::Left => Some(MouseButton::Left),
                            ::glutin::MouseButton::Middle => Some(MouseButton::Middle),
                            ::glutin::MouseButton::Right => Some(MouseButton::Right),
                            _ => None,
                        };
                        if let Some(button) = button {
                            let (x, y) = *MOUSE_POS.lock().unwrap();
                            events.push(match state {
                                            ::glutin::ElementState::Pressed => {
                                                Event::MouseDown { x, y, button }
                                            }
                                            ::glutin::ElementState::Released => {
                                                Event::MouseUp { x, y, button }
                                            }
                                        });
                        }
                    }
                    _ => {}
                }
            });
        EventIterator
    }

    pub fn should_close() -> bool {
        *SHOULD_CLOSE.lock().unwrap()
    }
}

pub(crate) use self::implementation::*;