use ::*;

#[derive(Debug, Copy, Clone)]
pub struct Mat4<T: Copy + Default = f64> {
    values: [T; 16],
}

impl<T: Copy + Default> Mat4<T> {
    pub fn transpose(self) -> Self {
        let mut result: Self = unsafe { std::mem::uninitialized() };
        for i in 0..4 {
            for j in 0..4 {
                result[(i, j)] = self[(j, i)]
            }
        }
        result
    }
}

impl<T: Copy + Default> Mat4<T> where T: std::ops::Mul<T, Output=T> + std::ops::Add<T, Output=T> + std::ops::Sub<T, Output=T> + std::ops::Div<T, Output=T> {
    pub fn inverse(self) -> Self {
        let a00 = self[(0, 0)];
        let a01 = self[(0, 1)];
        let a02 = self[(0, 2)];
        let a03 = self[(0, 3)];
        let a10 = self[(1, 0)];
        let a11 = self[(1, 1)];
        let a12 = self[(1, 2)];
        let a13 = self[(1, 3)];
        let a20 = self[(2, 0)];
        let a21 = self[(2, 1)];
        let a22 = self[(2, 2)];
        let a23 = self[(2, 3)];
        let a30 = self[(3, 0)];
        let a31 = self[(3, 1)];
        let a32 = self[(3, 2)];
        let a33 = self[(3, 3)];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;
        let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        Mat4 {
            values: [
                a11 * b11 - a12 * b10 + a13 * b09,
                a02 * b10 - a01 * b11 - a03 * b09,
                a31 * b05 - a32 * b04 + a33 * b03,
                a22 * b04 - a21 * b05 - a23 * b03,
                a12 * b08 - a10 * b11 - a13 * b07,
                a00 * b11 - a02 * b08 + a03 * b07,
                a32 * b02 - a30 * b05 - a33 * b01,
                a20 * b05 - a22 * b02 + a23 * b01,
                a10 * b10 - a11 * b08 + a13 * b06,
                a01 * b08 - a00 * b10 - a03 * b06,
                a30 * b04 - a31 * b02 + a33 * b00,
                a21 * b02 - a20 * b04 - a23 * b00,
                a11 * b07 - a10 * b09 - a12 * b06,
                a00 * b09 - a01 * b07 + a02 * b06,
                a31 * b01 - a30 * b03 - a32 * b00,
                a20 * b03 - a21 * b01 + a22 * b00]
        }.transpose() / det
    }
}

impl Mat4<f32> {
    pub fn identity() -> Self {
        let mut result = Self { values: [0.0; 16] };
        for i in 0..4 {
            result[(i, i)] = 1.0;
        }
        result
    }

    pub fn scale_uniform(factor: f32) -> Self {
        let mut result = Self { values: [0.0; 16] };
        result[(0, 0)] = factor;
        result[(1, 1)] = factor;
        result[(2, 2)] = factor;
        result[(3, 3)] = 1.0;
        result
    }

    pub fn scale(factor: Vec3<f32>) -> Self {
        let mut result = Self { values: [0.0; 16] };
        result[(0, 0)] = factor.x;
        result[(1, 1)] = factor.y;
        result[(2, 2)] = factor.z;
        result[(3, 3)] = 1.0;
        result
    }

    pub fn translate(dv: Vec3<f32>) -> Self {
        let mut result = Self::identity();
        result[(0, 3)] = dv.x;
        result[(1, 3)] = dv.y;
        result[(2, 3)] = dv.z;
        result
    }

    pub fn rotate_x(angle: f32) -> Self {
        let mut result = Self::identity();
        let cs = angle.cos();
        let sn = angle.sin();
        result[(1, 1)] = cs;
        result[(1, 2)] = -sn;
        result[(2, 1)] = sn;
        result[(2, 2)] = cs;
        result
    }

    pub fn rotate_y(angle: f32) -> Self {
        let mut result = Self::identity();
        let cs = angle.cos();
        let sn = angle.sin();
        result[(2, 2)] = cs;
        result[(2, 0)] = -sn;
        result[(0, 2)] = sn;
        result[(0, 0)] = cs;
        result
    }

    pub fn rotate_z(angle: f32) -> Self {
        let mut result = Self::identity();
        let cs = angle.cos();
        let sn = angle.sin();
        result[(0, 0)] = cs;
        result[(0, 1)] = -sn;
        result[(1, 0)] = sn;
        result[(1, 1)] = cs;
        result
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let ymax = near * (fov / 2.0).tan();
        let xmax = ymax * aspect;
        Self::frustum(-xmax, xmax, -ymax, ymax, near, far)
    }

    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let temp = 2.0 * near;
        let temp2 = right - left;
        let temp3 = top - bottom;
        let temp4 = far - near;
        Self {
            values: [
                temp / temp2,
                0.0,
                0.0,
                0.0,
                0.0,
                temp / temp3,
                0.0,
                0.0,
                (right + left) / temp2,
                (top + bottom) / temp3,
                (-far - near) / temp4,
                -1.0,
                0.0,
                0.0,
                (-temp * far) / temp4,
                0.0,
            ],
        }
    }
}

impl<T: Copy + Default> std::ops::Index<(usize, usize)> for Mat4<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.1 * 4 + index.0]
    }
}

impl<T: Copy + Default> std::ops::IndexMut<(usize, usize)> for Mat4<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.values[index.1 * 4 + index.0]
    }
}

impl std::ops::Mul for Mat4<f32> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut result = Self { values: [0.0; 16] };
        for i in 0..4 {
            for j in 0..4 {
                let cur = &mut result[(i, j)];
                for t in 0..4 {
                    *cur += self[(i, t)] * rhs[(t, j)];
                }
            }
        }
        result
    }
}

impl<T: Copy + Default, RHS: Copy> std::ops::Div<RHS> for Mat4<T>
    where T: std::ops::Div<RHS>, T::Output: Copy + Default {
    type Output = Mat4<T::Output>;

    fn div(self, rhs: RHS) -> Self::Output {
        let mut result: Self::Output = Mat4 { values: [T::Output::default(); 16] };
        for (s, d) in self.values.iter().zip(result.values.iter_mut()) {
            *d = *s / rhs;
        }
        result
    }
}

impl std::ops::Mul<Vec4<f32>> for Mat4<f32> {
    type Output = Vec4<f32>;

    fn mul(self, rhs: Vec4<f32>) -> Self::Output {
        let mul = |i| {
            self[(i, 0)] * rhs.x + self[(i, 1)] * rhs.y + self[(i, 2)] * rhs.z + self[(i, 3)] * rhs.w
        };
        vec4(mul(0), mul(1), mul(2), mul(3))
    }
}