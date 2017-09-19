use ::*;

#[derive(Debug)]
pub struct Unit {
    pub matrix: Mat4<f32>,
    pub head_position: Vec3<f32>,
    pub head_rotation: Vec2<f32>,
}

impl Unit {
    pub fn new() -> Self {
        Self {
            matrix: Mat4::identity(),
            head_rotation: vec2(0.0, 0.0),
            head_position: vec3(0.0, 1.0, 0.0),
        }
    }

    pub fn rotate_head(&mut self, rotation: Vec2<f32>) {
        self.head_rotation += rotation;
        self.head_rotation.y = self.head_rotation.y
            .min(std::f32::consts::PI / 2.0)
            .max(-std::f32::consts::PI / 2.0);
    }

    pub fn eye_matrix(&self) -> Mat4<f32> {
        Mat4::rotate_x(self.head_rotation.y) *
            Mat4::rotate_y(self.head_rotation.x) *
            Mat4::translate(-self.head_position) *
            self.matrix.inverse()
    }
}