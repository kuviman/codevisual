use std;

#[derive(Debug, Copy, Clone)]
pub struct Mat4<T: Copy = f64> {
    values: [T; 16],
}

impl<T: Copy> Mat4<T> {
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

impl Mat4<f32> {
    pub fn identity() -> Self {
        let mut result = Self { values: [0.0; 16] };
        for i in 0..4 {
            result[(i, i)] = 1.0;
        }
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
            values: [temp / temp2,
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
                     0.0],
        }
    }
}

impl<T: Copy> std::ops::Index<(usize, usize)> for Mat4<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0 * 4 + index.1]
    }
}

impl<T: Copy> std::ops::IndexMut<(usize, usize)> for Mat4<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.values[index.0 * 4 + index.1]
    }
}