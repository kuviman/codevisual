use ::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec2<T = f64> {
    pub x: T,
    pub y: T,
}

pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2 { x, y }
}

impl<T> Vec2<T> {
    pub fn extend(self, z: T) -> Vec3<T> {
        vec3(self.x, self.y, z)
    }
}

impl Vec2<f32> {
    pub fn normalize(self) -> Self {
        self / self.len()
    }
    pub fn len(self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
    pub fn rotated(v: Self, angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self {
            x: v.x * cos - v.y * sin,
            y: v.x * sin + v.y * cos,
        }
    }
}

impl Vec2<f64> {
    pub fn len(self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
}
