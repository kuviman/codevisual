use ::*;

#[cfg(target_os = "emscripten")]
mod impl_emscripten;

#[cfg(not(target_os = "emscripten"))]
mod impl_not_emscripten;

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

    LShift,
    RShift,

    Unknown,
}