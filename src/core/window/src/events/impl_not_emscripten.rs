use ::*;

impl Window {
    pub fn get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        {
            let mut handle_event = |e| match e {
                glutin::WindowEvent::Closed => self.should_close.set(true),
                glutin::WindowEvent::MouseWheel { delta, .. } => {
                    events.push(Event::Wheel {
                                    delta: match delta {
                                        glutin::MouseScrollDelta::PixelDelta(_, dy) => dy as f64,
                                        glutin::MouseScrollDelta::LineDelta(_, dy) => {
                                            dy as f64 * 51.0
                                        }
                                    },
                                });
                }
                glutin::WindowEvent::MouseMoved { position: (x, y), .. } => {
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
                                        glutin::ElementState::Pressed => {
                                            Event::MouseDown { position, button }
                                        }
                                        glutin::ElementState::Released => {
                                            Event::MouseUp { position, button }
                                        }
                                    });
                    }
                }
                _ => {}
            };
            self.glutin_events_loop
                .borrow_mut()
                .poll_events(|e| if let glutin::Event::WindowEvent { event: e, .. } = e {
                                 handle_event(e)
                             });
        }
        events
    }
}