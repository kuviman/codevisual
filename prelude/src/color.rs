#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => { rgb!($r, $g, $b, 1.0) };
    ($r:expr, $g:expr, $b:expr, $a: expr) => {
        Color {
            red: $r,
            green: $g,
            blue: $b,
            alpha: $a,
        }
    };
}

impl Color {
    pub const BLACK: Color = rgb!(0.0, 0.0, 0.0);
    pub const WHITE: Color = rgb!(1.0, 1.0, 1.0);

    pub const RED: Color = rgb!(1.0, 0.0, 0.0);
    pub const GREEN: Color = rgb!(0.0, 1.0, 0.0);
    pub const BLUE: Color = rgb!(0.0, 0.0, 1.0);

    pub const YELLOW: Color = rgb!(1.0, 1.0, 0.0);
    pub const MAGENTA: Color = rgb!(1.0, 0.0, 1.0);
    pub const CYAN: Color = rgb!(0.0, 1.0, 1.0);

    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Color {
            red,
            green,
            blue,
            alpha: 1.0,
        }
    }
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn argb_hex(value: u32) -> Self {
        Color {
            red: ((value >> 16) & 0xff) as f32 / 255.0,
            green: ((value >> 8) & 0xff) as f32 / 255.0,
            blue: (value & 0xff) as f32 / 255.0,
            alpha: (value >> 24) as f32 / 255.0,
        }
    }

    pub fn mix(color1: Self, color2: Self, k1: f32, k2: f32) -> Self {
        let sum = k1 + k2;
        let k1 = k1 / sum;
        let k2 = k2 / sum;
        Self {
            red: color1.red * k1 + color2.red * k2,
            green: color1.green * k1 + color2.green * k2,
            blue: color1.blue * k1 + color2.blue * k2,
            alpha: color1.alpha * k1 + color2.alpha * k2,
        }
    }
}
