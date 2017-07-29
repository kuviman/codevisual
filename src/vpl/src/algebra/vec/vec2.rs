use std;

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T: Copy = f64> {
    pub x: T,
    pub y: T,
}

pub fn vec2<T: Copy>(x: T, y: T) -> Vec2<T> {
    Vec2 { x, y }
}

impl<T: Copy + std::ops::Add<T, Output = T>> std::ops::Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Copy + std::ops::AddAssign<T>> std::ops::AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Copy + std::ops::Sub<T, Output = T>> std::ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Copy + std::ops::Mul<T, Output = T>> std::ops::Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Copy + std::ops::Div<T, Output = T>> std::ops::Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
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
