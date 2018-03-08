use ::*;

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
mod impl_web;

#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
mod impl_default;

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct TouchPoint {
    pub position: Vec2,
}

#[derive(Debug, Clone)]
pub enum Event {
    MouseDown { position: Vec2, button: MouseButton },
    MouseUp { position: Vec2, button: MouseButton },
    MouseMove { position: Vec2 },
    Wheel { delta: f64 },
    TouchStart { touches: Vec<TouchPoint> },
    TouchMove { touches: Vec<TouchPoint> },
    TouchEnd,
    KeyDown { key: Key },
    KeyUp { key: Key },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,
    Space,
    Enter,
    Backspace,

    LShift,
    RShift,

    LCtrl,
    RCtrl,

    LAlt,
    RAlt,

    Left,
    Right,
    Up,
    Down,

    PageUp,
    PageDown,

    Unknown,
}
