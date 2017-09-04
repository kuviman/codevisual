use ::*;

#[derive(Uniforms, Debug, Clone)]
pub struct CameraUniforms {
    u_projection_matrix: Mat4<f32>,
    u_view_matrix: Mat4<f32>,
}

const MIN_ATTACK_ANGLE: f32 = 0.5;
const MAX_ATTACK_ANGLE: f32 = std::f32::consts::PI / 2.0;
const DEFAULT_ATTACK_ANGLE: f32 = MIN_ATTACK_ANGLE * 0.25 + MAX_ATTACK_ANGLE * 0.75;

const MAX_DISTANCE: f32 = 5.0;
const MIN_DISTANCE: f32 = 1.0;

pub struct Camera {
    app: Rc<codevisual::Application>,
    fov: f32,
    position: Vec3<f32>,
    distance: f32,
    rotation: f32,
    attack_angle: f32,
    start_drag: Option<Vec2>,
    start_drag_rotation: Option<Vec2>,
    prev_zoom_touchdist: f32,
}

impl Camera {
    pub fn new(app: &Rc<codevisual::Application>) -> Self {
        Self {
            fov: std::f32::consts::PI / 2.0,
            app: app.clone(),
            position: vec3(0.0, 0.0, 0.0),
            distance: MAX_DISTANCE,
            rotation: 0.1,
            attack_angle: DEFAULT_ATTACK_ANGLE,
            start_drag: None,
            start_drag_rotation: None,
            prev_zoom_touchdist: 0.0,
        }
    }

    pub fn projection_matrix(&self) -> Mat4<f32> {
        let window_size = self.app.window().get_size();
        Mat4::perspective(self.fov / 2.0, window_size.x as f32 / window_size.y as f32, 1e-1, 1e5)
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

    pub fn handle(&mut self, event: codevisual::Event) {
        use codevisual::Event::*;
        match event {
            MouseDown {
                position,
                button: codevisual::MouseButton::Left,
            } => {
                self.app.window().set_cursor_type(
                    codevisual::CursorType::Drag,
                );
                self.start_drag = Some(position);
            }
            MouseDown {
                position,
                button: codevisual::MouseButton::Right,
            } => {
                self.app.window().set_cursor_type(
                    codevisual::CursorType::Drag,
                );
                self.start_drag_rotation = Some(position);
            }
            MouseMove { position: Vec2 { x, y } } => {
                if let Some(Vec2 {
                                x: prev_x,
                                y: prev_y,
                            }) = self.start_drag
                    {
                        let dv = vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                            self.app.window().get_size().y as f32;
                        let dv = vec2(dv.x, dv.y / self.attack_angle.cos());
                        let dv = Vec2::rotated(dv, -self.rotation);
                        self.position.x += dv.x * self.distance;
                        self.position.y += dv.y * self.distance;
                        self.start_drag = Some(vec2(x, y));
                    }
                if let Some(Vec2 {
                                x: prev_x,
                                y: prev_y,
                            }) = self.start_drag_rotation
                    {
                        const SENS: f64 = 2.0;
                        let dv = vec2(x - prev_x, y - prev_y) * SENS /
                            self.app.window().get_size().y as f64;
                        self.rotation += dv.x as f32;
                        self.attack_angle = (self.attack_angle + dv.y as f32)
                            .min(MAX_ATTACK_ANGLE).max(MIN_ATTACK_ANGLE);
                        self.start_drag_rotation = Some(vec2(x, y));
                    }
            }
            MouseUp { .. } => {
                self.app.window().set_cursor_type(
                    codevisual::CursorType::Default,
                );
                self.start_drag = None;
                self.start_drag_rotation = None;
            }
            TouchStart { touches } => {
                if touches.len() == 1 {
                    self.start_drag = Some(touches[0].position);
                }
                if touches.len() == 2 {
                    self.prev_zoom_touchdist = (touches[0].position - touches[1].position)
                        .len() as f32;
                }
            }
            TouchMove { touches } => {
                if touches.len() == 1 {
                    let Vec2 { x, y } = touches[0].position;
                    if let Some(Vec2 {
                                    x: prev_x,
                                    y: prev_y,
                                }) = self.start_drag
                        {
                            let dv = vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                                self.app.window().get_size().y as f32 *
                                self.distance;
                            self.position.x += dv.x;
                            self.position.y += dv.y;
                            self.start_drag = Some(vec2(x, y));
                        }
                } else if touches.len() == 2 {
                    let now_dist = (touches[0].position - touches[1].position).len() as f32;
                    self.distance /= now_dist / self.prev_zoom_touchdist;
                    self.prev_zoom_touchdist = now_dist;
                }
            }
            TouchEnd => {
                self.start_drag = None;
            }
            Wheel { delta } => {
                self.distance *= f32::exp(delta as f32 / 1000.0);
            }
            _ => (),
        }
        self.distance = self.distance.min(MAX_DISTANCE).max(MIN_DISTANCE);
    }
}