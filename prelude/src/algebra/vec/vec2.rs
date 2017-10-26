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

impl<T: Num + Copy> Vec2<T> {
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y
    }
    pub fn cross(a: Self, b: Self) -> T {
        a.x * b.y - a.y * b.x
    }
}

impl<T: Float> Vec2<T> {
    pub fn normalize(self) -> Self {
        self / self.len()
    }
    pub fn len(self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y)
    }
    pub fn rotated(v: Self, angle: T) -> Self {
        let (sin, cos) = T::sin_cos(angle);
        Self {
            x: v.x * cos - v.y * sin,
            y: v.x * sin + v.y * cos,
        }
    }
}