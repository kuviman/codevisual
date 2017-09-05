use ::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec4<T: Copy = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub fn vec4<T: Copy>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 { x, y, z, w }
}

impl<T: Copy + std::ops::Neg> std::ops::Neg for Vec4<T> where T::Output: Copy {
    type Output = Vec4<T::Output>;

    fn neg(self) -> Self::Output {
        vec4(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<T: Copy + std::ops::Div<Output=T>> std::ops::Div<T> for Vec4<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl<T: Copy + std::ops::Add<Output=T>> std::ops::Add for Vec4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T: Copy + std::ops::Sub<Output=T>> std::ops::Sub for Vec4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T: Copy + std::ops::Mul<T, Output=T>> std::ops::Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
