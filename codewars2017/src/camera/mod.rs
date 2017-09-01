use ::*;

#[derive(Uniforms, Debug, Clone)]
pub struct CameraUniforms {
    u_projection_matrix: Mat4<f32>,
    u_view_matrix: Mat4<f32>,
}

const MIN_ATTACK_ANGLE: f32 = 0.5;
const MAX_ATTACK_ANGLE: f32 = std::f32::consts::PI / 2.0 - 0.5;

const MAX_DISTANCE: f32 = 5.0;
const MIN_DISTANCE: f32 = 1.0;

pub struct Camera {
    fov: f32,
    aspect: f32,
    position: Vec3<f32>,
    distance: f32,
    rotation: f32,
    attack_angle: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            fov: std::f32::consts::PI / 2.0,
            aspect: 1.0,
            position: vec3(0.0, 0.0, 0.0),
            distance: MAX_DISTANCE,
            rotation: 0.1,
            attack_angle: MAX_ATTACK_ANGLE,
        }
    }

    pub fn update(&mut self, viewport_size: Vec2<f32>) {
        self.aspect = viewport_size.x as f32 / viewport_size.y as f32;
    }

    pub fn projection_matrix(&self) -> Mat4<f32> {
        Mat4::perspective(self.fov / 2.0, self.aspect, 1e-1, 1e5)
    }

    pub fn view_matrix(&self) -> Mat4<f32> {
        Mat4::translate(vec3(0.0, 0.0, -self.distance)) *
            Mat4::rotate_x(self.attack_angle - std::f32::consts::PI / 2.0) *
            Mat4::rotate_z(self.rotation) *
            Mat4::translate(self.position) *
            Mat4::scale(vec3(1.0, -1.0, 1.0))
    }

    pub fn matrix(&self) -> Mat4<f32> {
        self.projection_matrix() * self.view_matrix()
    }

    pub fn uniforms(&self) -> CameraUniforms {
        CameraUniforms {
            u_projection_matrix: self.projection_matrix(),
            u_view_matrix: self.view_matrix(),
        }
    }
}