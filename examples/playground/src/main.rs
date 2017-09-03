#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;

mod obj;

mod ground;

use ground::Ground;

mod units;

use units::AllUnits as Units;

mod decor;

use decor::AllDecor as Decor;

mod clouds;

use clouds::Clouds;

mod settings;

pub ( crate ) use settings::*;

mod fog;

mod minimap;

pub ( crate ) use minimap::*;

shader_library! {
    ShaderLib {
        "global" => include_str!("global.glsl"),
        "format/obj" => include_str!("obj/lib.glsl"),
        "ground" => include_str!("ground/lib.glsl"),
        "fog" => include_str!("fog/lib.glsl"),
        "units" => include_str!("units/lib.glsl"),
    }
}

const MAP_SIZE: f32 = 2000.0;
const TICK_TIME: f32 = 0.016666;
const MIN_CAMERA_DIST: f32 = 150.0;
const MAX_CAMERA_DIST: f32 = 2000.0;

#[derive(Uniforms)]
pub struct GlobalUniforms {
    u_time: f32,
    u_matrix: Mat4<f32>,
    u_map_size: f32,
    u_camera_matrix: Mat4<f32>,
    u_projection_matrix: Mat4<f32>,
    u_screen_size: Vec2<f32>,
}

pub struct Playground {
    app: Rc<codevisual::Application>,

    fog: fog::Fog,
    units: Units,
    ground: Ground,
    decor: Decor,
    clouds: Clouds,
    minimap: Minimap,

    global_uniforms: GlobalUniforms,

    current_time: f32,

    camera_distance: f32,
    camera_position: Vec2<f32>,
    camera_rotation: Vec2<f32>,

    rotate_mouse_pos: Option<Vec2>,

    start_drag: Option<Vec2>,
    prev_zoom_touchdist: f32,

    settings: Rc<Settings>,
}

resources! {
    Resources {
        units: units::Resources = (),
        ground: ground::Resources = (),
        decor: decor::Resources = (),
        clouds: clouds::Resources = (),
    }
}

impl codevisual::Game for Playground {
    type Resources = Resources;

    fn get_title() -> String {
        String::from("CodeVisual Playground")
    }

    fn new(app: Rc<codevisual::Application>, resources: Resources) -> Self {
        app.window().set_cursor_type(
            codevisual::CursorType::Pointer,
        );
        let settings = Rc::new(Settings::new(&app));
        let decor = Decor::new(
            &app,
            resources.decor,
            &resources.ground.map_texture,
            &settings,
        );
        let clouds = Clouds::new(
            &app,
            resources.clouds,
            &settings,
        );
        Self {
            app: app.clone(),

            fog: fog::Fog::new(&app, &settings),
            units: Units::new(&app, resources.units, &settings),
            ground: Ground::new(&app, resources.ground, &settings),
            decor,
            clouds,
            minimap: Minimap::new(&app, &settings),

            global_uniforms: GlobalUniforms {
                u_time: 0.0,
                u_matrix: Mat4::identity(),
                u_map_size: MAP_SIZE,
                u_camera_matrix: Mat4::identity(),
                u_projection_matrix: Mat4::identity(),
                u_screen_size: vec2(1.0, 1.0),
            },

            current_time: 0.0,

            camera_distance: MAX_CAMERA_DIST / 2.0,
            camera_position: vec2(0.0, 0.0),
            camera_rotation: vec2(0.0, -0.3),

            rotate_mouse_pos: None,

            start_drag: None,
            prev_zoom_touchdist: 0.0,

            settings: settings.clone(),
        }
    }

    fn update(&mut self, delta_time: f64) {
        let mut delta_time = delta_time as f32;
        delta_time *= self.settings.time_scale.get() as f32;
        self.current_time += delta_time;
        self.units.update(delta_time);
    }

    fn draw(&mut self) {
        self.global_uniforms.u_screen_size = {
            let size = self.app.window().get_size();
            vec2(size.x as f32, size.y as f32)
        };
        let mut framebuffer = ugli::default_framebuffer(self.app.ugli_context());
        ugli::clear(&mut framebuffer, Some(Color::rgb(1.0, 1.0, 1.0)), Some(1.0));

        self.global_uniforms.u_time = self.current_time;
        self.global_uniforms.u_projection_matrix = {
            let Vec2 { x: w, y: h } = self.app.window().get_size();
            Mat4::perspective(std::f32::consts::PI / 4.0, w as f32 / h as f32, 1.0, 5500.0)
        };
        self.global_uniforms.u_camera_matrix = Mat4::translate(
            vec3(0.0, 0.0, -self.camera_distance),
        ) * Mat4::rotate_x(self.camera_rotation.y) *
            Mat4::rotate_z(self.camera_rotation.x) *
            Mat4::translate(vec3(self.camera_position.x, self.camera_position.y, 0.0));
        self.global_uniforms.u_matrix = self.global_uniforms.u_projection_matrix *
            self.global_uniforms.u_camera_matrix;
        if self.settings.fog_enabled.get() {
            self.fog.prepare(&self.units, &self.global_uniforms);
        }
        let uniforms = (&self.global_uniforms, &self.fog.uniforms);
        self.units.draw(&mut framebuffer, &(
            &uniforms,
            &self.ground.uniforms,
        ));
        self.ground.draw(&mut framebuffer, &uniforms);
        {
            let uniforms = (
                &uniforms,
                uniforms! {
                    u_screen_used_texture: if self.settings.decor_transparency.get() {
                        Some(self.units.get_screen_used_texture(
                            &(&self.global_uniforms, &self.ground.uniforms),
                        ))
                    } else {
                        None
                    },
                    FRAMEBUFFER_SIZE: {
                        let size = framebuffer.get_size();
                        vec2(size.x as f32, size.y as f32)
                    },
                },
            );
            self.decor.draw(&mut framebuffer, &(
                &uniforms,
                &self.ground.uniforms,
            ));
        }
        if self.settings.clouds_enabled.get() {
            self.clouds.draw(&mut framebuffer, &uniforms);
        }
        self.minimap.render(
            &mut framebuffer,
            &self.units,
            &uniforms,
        );
    }

    fn handle_event(&mut self, event: codevisual::Event) {
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
                self.rotate_mouse_pos = Some(position);
            }
            MouseMove { position: Vec2 { x, y } } => {
                if let Some(Vec2 {
                                x: prev_x,
                                y: prev_y,
                            }) = self.start_drag
                    {
                        let dv = vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                            self.app.window().get_size().y as f32;
                        let dv = vec2(dv.x, dv.y / self.camera_rotation.y.cos());
                        let dv = Vec2::rotated(dv, -self.camera_rotation.x);
                        self.camera_position += dv * self.camera_distance;
                        self.start_drag = Some(vec2(x, y));
                    }
                if let Some(Vec2 {
                                x: prev_x,
                                y: prev_y,
                            }) = self.rotate_mouse_pos
                    {
                        const SENS: f64 = 2.0;
                        let dv = vec2(x - prev_x, y - prev_y) * SENS /
                            self.app.window().get_size().y as f64;
                        self.camera_rotation.x += dv.x as f32;
                        self.camera_rotation.y = (self.camera_rotation.y + dv.y as f32).min(0.0).max(
                            -std::f32::consts::PI /
                                3.0,
                        );
                        self.rotate_mouse_pos = Some(vec2(x, y));
                    }
            }
            MouseUp { .. } => {
                self.app.window().set_cursor_type(
                    codevisual::CursorType::Pointer,
                );
                self.start_drag = None;
                self.rotate_mouse_pos = None;
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
                            self.camera_position += vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                                self.app.window().get_size().y as f32 *
                                self.camera_distance;
                            self.start_drag = Some(vec2(x, y));
                        }
                } else if touches.len() == 2 {
                    let now_dist = (touches[0].position - touches[1].position).len() as f32;
                    self.camera_distance /= now_dist / self.prev_zoom_touchdist;
                    self.prev_zoom_touchdist = now_dist;
                }
            }
            TouchEnd => {
                self.start_drag = None;
            }
            Wheel { delta } => {
                self.camera_distance *= f32::exp(delta as f32 / 1000.0);
            }
            _ => (),
        }
        self.camera_distance = self.camera_distance.min(MAX_CAMERA_DIST).max(
            MIN_CAMERA_DIST,
        );
    }
}

fn main() {
    #[cfg(not(target_os = "emscripten"))]
    std::env::set_current_dir("examples/playground/static").unwrap();
    codevisual::run::<Playground>();
}
