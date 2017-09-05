use ::*;

#[derive(Debug)]
pub struct Unit {
    matrix: Mat4<f32>,
    head_rotation: Vec2<f32>,
}

impl Unit {
    pub fn new() -> Self {
        Self {
            matrix: Mat4::identity(),
            head_rotation: vec2(0.0, 0.0),
        }
    }
    pub fn get_matrix(&self) -> Mat4<f32> {
        self.matrix
    }
    pub fn get_head_rotation(&self) -> Vec2<f32> {
        self.head_rotation
    }
    pub fn get_eye_matrix(&self) -> Mat4<f32> {
        Mat4::rotate_x(self.head_rotation.y) *
            Mat4::rotate_y(self.head_rotation.x) *
            self.matrix.inverse()
    }
}