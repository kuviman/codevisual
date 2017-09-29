use ::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec3<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x, y, z }
}

impl<T> Vec3<T> {
    pub fn extend(self, w: T) -> Vec4<T> {
        vec4(self.x, self.y, self.z, w)
    }
}

impl<T: Copy + num::Num> Vec3<T> {
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}