use ::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec4<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 { x, y, z, w }
}

impl<T: Copy + num::Num> Vec4<T> {
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}