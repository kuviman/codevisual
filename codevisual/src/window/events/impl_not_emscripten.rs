use ::*;

impl Window {
    pub ( crate ) fn internal_get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        {
            let mut handle_event = |e| match e {
                glutin::WindowEvent::Closed => self.should_close.set(true),
                glutin::WindowEvent::MouseWheel { delta, .. } => {
                    events.push(Event::Wheel {
                        delta: match delta {
                            glutin::MouseScrollDelta::PixelDelta(_, dy) => dy as f64,
                            glutin::MouseScrollDelta::LineDelta(_, dy) => dy as f64 * 51.0,
                        },
                    });
                }
                glutin::WindowEvent::CursorMoved { position: (x, y), .. } => {
                    let position = vec2(x as f64, y as f64);
                    self.mouse_pos.set(position);
                    events.push(Event::MouseMove { position })
                }
                glutin::WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        glutin::MouseButton::Left => Some(MouseButton::Left),
                        glutin::MouseButton::Middle => Some(MouseButton::Middle),
                        glutin::MouseButton::Right => Some(MouseButton::Right),
                        _ => None,
                    };
                    if let Some(button) = button {
                        let position = self.mouse_pos.get();
                        events.push(match state {
                            glutin::ElementState::Pressed => Event::MouseDown { position, button },
                            glutin::ElementState::Released => Event::MouseUp { position, button },
                        });
                    }
                }
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        let key = from_glutin_key(key);
                        events.push(match input.state {
                            glutin::ElementState::Pressed => {
                                Event::KeyDown { key: key }
                            }
                            glutin::ElementState::Released => {
                                Event::KeyUp { key: key }
                            }
                        });
                    }
                }
                _ => {}
            };
            self.glutin_events_loop.borrow_mut().poll_events(|e| {
                if let glutin::Event::WindowEvent { event: e, .. } = e {
                    handle_event(e)
                }
            });
        }
        events
    }
}

fn from_glutin_key(key: glutin::VirtualKeyCode) -> Key {
    use glutin::VirtualKeyCode as GKey;
    match key {
        GKey::Key0 => Key::Num0,
        GKey::Key1 => Key::Num1,
        GKey::Key2 => Key::Num2,
        GKey::Key3 => Key::Num3,
        GKey::Key4 => Key::Num4,
        GKey::Key5 => Key::Num5,
        GKey::Key6 => Key::Num6,
        GKey::Key7 => Key::Num7,
        GKey::Key8 => Key::Num8,
        GKey::Key9 => Key::Num9,

        GKey::A => Key::A,
        GKey::B => Key::B,
        GKey::C => Key::C,
        GKey::D => Key::D,
        GKey::E => Key::E,
        GKey::F => Key::F,
        GKey::G => Key::G,
        GKey::H => Key::H,
        GKey::I => Key::I,
        GKey::J => Key::J,
        GKey::K => Key::K,
        GKey::L => Key::L,
        GKey::M => Key::M,
        GKey::N => Key::N,
        GKey::O => Key::O,
        GKey::P => Key::P,
        GKey::Q => Key::Q,
        GKey::R => Key::R,
        GKey::S => Key::S,
        GKey::T => Key::T,
        GKey::U => Key::U,
        GKey::V => Key::V,
        GKey::W => Key::W,
        GKey::X => Key::X,
        GKey::Y => Key::Y,
        GKey::Z => Key::Z,

        GKey::Escape => Key::Escape,
        GKey::Space => Key::Space,
        GKey::Return => Key::Enter,
        GKey::Back => Key::Backspace,

        GKey::LShift => Key::LShift,
        GKey::RShift => Key::RShift,

        GKey::LControl => Key::LCtrl,
        GKey::RControl => Key::RCtrl,

        GKey::LAlt => Key::LAlt,
        GKey::RAlt => Key::RAlt,

        GKey::Left => Key::Left,
        GKey::Right => Key::Right,
        GKey::Up => Key::Up,
        GKey::Down => Key::Down,

        GKey::PageUp => Key::PageUp,
        GKey::PageDown => Key::PageDown,

        _ => {
//            eprintln!("Unrecognized key: {:?}", key);
            Key::Unknown
        }
    }
}