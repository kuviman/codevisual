use ::*;

#[derive(Debug, Copy, Clone)]
pub struct Mat4<T: Num + Copy = f64> {
    values: [T; 16],
}

impl<T: Num + Copy> Index<(usize, usize)> for Mat4<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.1 * 4 + index.0]
    }
}

impl<T: Num + Copy> IndexMut<(usize, usize)> for Mat4<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.values[index.1 * 4 + index.0]
    }
}

impl<T: Num + Copy> Add for Mat4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<T: Num + Copy> AddAssign for Mat4<T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..16 {
            self.values[i] = self.values[i] + rhs.values[i];
        }
    }
}

impl<T: Num + Copy> Sub for Mat4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl<T: Num + Copy> SubAssign for Mat4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..16 {
            self.values[i] = self.values[i] - rhs.values[i];
        }
    }
}

impl<T: Num + Copy + Neg<Output=T>> Neg for Mat4<T> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut result = self;
        for i in 0..16 {
            result.values[i] = -self.values[i];
        }
        result
    }
}

impl<T: Num + Copy> Mul for Mat4<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut result = Self { values: [T::zero(); 16] };
        for i in 0..4 {
            for j in 0..4 {
                let cur = &mut result[(i, j)];
                for t in 0..4 {
                    *cur = *cur + self[(i, t)] * rhs[(t, j)];
                }
            }
        }
        result
    }
}

impl<T: Num + Copy> MulAssign for Mat4<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Num + Copy> Mul<T> for Mat4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<T: Num + Copy> MulAssign<T> for Mat4<T> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..16 {
            self.values[i] = self.values[i] * rhs;
        }
    }
}

impl<T: Num + Copy> Div<T> for Mat4<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl<T: Num + Copy> DivAssign<T> for Mat4<T> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..16 {
            self.values[i] = self.values[i] * rhs;
        }
    }
}

impl<T: Float> Mul<Vec4<T>> for Mat4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Vec4<T> {
        let mul = |i| {
            self[(i, 0)] * rhs.x + self[(i, 1)] * rhs.y + self[(i, 2)] * rhs.z + self[(i, 3)] * rhs.w
        };
        vec4(mul(0), mul(1), mul(2), mul(3))
    }
}

impl<T: Num + Copy> Mat4<T> {
    pub fn zero() -> Self {
        Self { values: [T::zero(); 16] }
    }
    pub fn identity() -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            result[(i, i)] = T::one();
        }
        result
    }

    pub fn transpose(self) -> Self {
        let mut result = self;
        for i in 0..4 {
            for j in 0..4 {
                result[(i, j)] = self[(j, i)]
            }
        }
        result
    }

    pub fn scale_uniform(factor: T) -> Self {
        let mut result = Self::zero();
        result[(0, 0)] = factor;
        result[(1, 1)] = factor;
        result[(2, 2)] = factor;
        result[(3, 3)] = T::one();
        result
    }
    pub fn scale(factor: Vec3<T>) -> Self {
        let mut result = Self::zero();
        result[(0, 0)] = factor.x;
        result[(1, 1)] = factor.y;
        result[(2, 2)] = factor.z;
        result[(3, 3)] = T::one();
        result
    }

    pub fn translate(dv: Vec3<T>) -> Self {
        let mut result = Self::identity();
        result[(0, 3)] = dv.x;
        result[(1, 3)] = dv.y;
        result[(2, 3)] = dv.z;
        result
    }
}

impl<T: Float> Mat4<T> {
    pub fn inverse(self) -> Self {
        let a = &self.values;
        let a00 = a[0];
        let a01 = a[1];
        let a02 = a[2];
        let a03 = a[3];
        let a10 = a[4];
        let a11 = a[5];
        let a12 = a[6];
        let a13 = a[7];
        let a20 = a[8];
        let a21 = a[9];
        let a22 = a[10];
        let a23 = a[11];
        let a30 = a[12];
        let a31 = a[13];
        let a32 = a[14];
        let a33 = a[15];

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

        let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        if det == T::zero() {
            Self::identity()
        } else {
            det = T::one() / det;

            Self {
                values: [
                    (a11 * b11 - a12 * b10 + a13 * b09) * det,
                    (a02 * b10 - a01 * b11 - a03 * b09) * det,
                    (a31 * b05 - a32 * b04 + a33 * b03) * det,
                    (a22 * b04 - a21 * b05 - a23 * b03) * det,
                    (a12 * b08 - a10 * b11 - a13 * b07) * det,
                    (a00 * b11 - a02 * b08 + a03 * b07) * det,
                    (a32 * b02 - a30 * b05 - a33 * b01) * det,
                    (a20 * b05 - a22 * b02 + a23 * b01) * det,
                    (a10 * b10 - a11 * b08 + a13 * b06) * det,
                    (a01 * b08 - a00 * b10 - a03 * b06) * det,
                    (a30 * b04 - a31 * b02 + a33 * b00) * det,
                    (a21 * b02 - a20 * b04 - a23 * b00) * det,
                    (a11 * b07 - a10 * b09 - a12 * b06) * det,
                    (a00 * b09 - a01 * b07 + a02 * b06) * det,
                    (a31 * b01 - a30 * b03 - a32 * b00) * det,
                    (a20 * b03 - a21 * b01 + a22 * b00) * det, ]
            }
        }
    }

    pub fn rotate_x(angle: T) -> Self {
        let mut result = Self::identity();
        let cs = angle.cos();
        let sn = angle.sin();
        result[(1, 1)] = cs;
        result[(1, 2)] = -sn;
        result[(2, 1)] = sn;
        result[(2, 2)] = cs;
        result
    }
    pub fn rotate_y(angle: T) -> Self {
        let mut result = Self::identity();
        let cs = angle.cos();
        let sn = angle.sin();
        result[(2, 2)] = cs;
        result[(2, 0)] = -sn;
        result[(0, 2)] = sn;
        result[(0, 0)] = cs;
        result
    }
    pub fn rotate_z(angle: T) -> Self {
        let mut result = Self::identity();
        let cs = angle.cos();
        let sn = angle.sin();
        result[(0, 0)] = cs;
        result[(0, 1)] = -sn;
        result[(1, 0)] = sn;
        result[(1, 1)] = cs;
        result
    }

    pub fn perspective(fov: T, aspect: T, near: T, far: T) -> Self {
        let ymax = near * (fov / (T::one() + T::one())).tan();
        let xmax = ymax * aspect;
        Self::frustum(-xmax, xmax, -ymax, ymax, near, far)
    }

    pub fn frustum(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let double_near = near + near;
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        Self {
            values: [
                double_near / width,
                T::zero(),
                T::zero(),
                T::zero(),
                T::zero(),
                double_near / height,
                T::zero(),
                T::zero(),
                (right + left) / width,
                (top + bottom) / height,
                (-far - near) / depth,
                -T::one(),
                T::zero(),
                T::zero(),
                (-double_near * far) / depth,
                T::zero(),
            ],
        }
    }
}