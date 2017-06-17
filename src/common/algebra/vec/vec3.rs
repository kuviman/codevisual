use std;

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T: Copy = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub fn vec3<T: Copy>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x, y, z }
}
impl<T: Copy + std::ops::Div<Output = T>> std::ops::Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}