use ::*;

use stdweb::traits::{IKeyboardEvent, IMouseEvent};
use stdweb::web::event as we;

trait Convert<T>: Sized {
    fn convert(value: T) -> Option<Self>;
}

impl Convert<String> for Key {
    fn convert(key: String) -> Option<Key> {
        use Key::*;
        Some(match key.as_str() {
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

            _ => return None,
        })
    }
}

impl Convert<we::MouseButton> for MouseButton {
    fn convert(button: we::MouseButton) -> Option<MouseButton> {
        Some(match button {
            we::MouseButton::Left => MouseButton::Left,
            we::MouseButton::Wheel => MouseButton::Middle,
            we::MouseButton::Right => MouseButton::Right,
            _ => return None,
        })
    }
}

impl Convert<we::KeyDownEvent> for Event {
    fn convert(event: we::KeyDownEvent) -> Option<Event> {
        Convert::convert(event.code()).map(|key| Event::KeyDown { key })
    }
}

impl Convert<we::KeyUpEvent> for Event {
    fn convert(event: we::KeyUpEvent) -> Option<Event> {
        Convert::convert(event.code()).map(|key| Event::KeyUp { key })
    }
}

impl Convert<we::MouseDownEvent> for Event {
    fn convert(event: we::MouseDownEvent) -> Option<Event> {
        Convert::convert(event.button()).map(|button| Event::MouseDown {
            position: vec2(event.offset_x(), event.offset_y()),
            button,
        })
    }
}

impl Convert<we::MouseUpEvent> for Event {
    fn convert(event: we::MouseUpEvent) -> Option<Event> {
        Convert::convert(event.button()).map(|button| Event::MouseUp {
            position: vec2(event.offset_x(), event.offset_y()),
            button,
        })
    }
}

impl Convert<we::MouseMoveEvent> for Event {
    fn convert(event: we::MouseMoveEvent) -> Option<Event> {
        Some(Event::MouseMove {
            position: vec2(event.offset_x(), event.offset_y()),
        })
    }
}

impl Window {
    pub(crate) fn subscribe_events<F: Fn(Event) + 'static>(&self, handler: F) {
        use stdweb::web::{IEventTarget, IHtmlElement};
        let handler = Rc::new(handler);
        macro_rules! setup_event {
            ($canvas:expr, $handler:expr, $event:ty) => {
                let handler = handler.clone();
                let canvas_clone = $canvas.clone();
                $canvas.add_event_listener(move |event: $event| {
                    canvas_clone.focus();
                    if let Some(event) = Convert::convert(event) {
                        handler(event);
                    }
                });
            };
        }
        setup_event!(self.canvas, handler, we::KeyDownEvent);
        setup_event!(self.canvas, handler, we::KeyUpEvent);
        setup_event!(self.canvas, handler, we::MouseDownEvent);
        setup_event!(self.canvas, handler, we::MouseUpEvent);
        setup_event!(self.canvas, handler, we::MouseMoveEvent);
    }
}
