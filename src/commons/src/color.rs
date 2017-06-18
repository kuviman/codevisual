#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
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
}