#[macro_use]
extern crate codevisual;

mod obj;
mod ground;
mod units;

use ground::Ground;
use units::Units;

use codevisual::commons::*;
use codevisual::draw;

const MAP_SIZE: f32 = 1000.0;
const TICK_TIME: f32 = 0.016666;
const MIN_CAMERA_DIST: f32 = 150.0;
const MAX_CAMERA_DIST: f32 = 2000.0;

#[derive(Uniforms)]
pub struct GlobalUniforms {
    u_time: f32,
    u_matrix: Mat4<f32>,
}

pub struct Playground {
    app: Rc<codevisual::Application>,
    current_time: f32,
    units: Units,
    ground: Ground,
    global_uniforms: GlobalUniforms,
    start_drag: Option<Vec2>,
    prev_zoom_touchdist: f32,
    camera_distance: f32,
    camera_position: Vec2<f32>,
    time_scale: Rc<Cell<f32>>,
}

pub struct Resources {
    pub car_texture: draw::TextureResource,
    pub heli_texture: draw::TextureResource,
    pub dirt_texture: draw::TextureResource,
    pub grass_texture: draw::TextureResource,
    pub darkgrass_texture: draw::TextureResource,
    pub map_texture: draw::TextureResource,
    pub bush_texture: draw::TextureResource,
    pub car_obj: codevisual::TextResource,
    pub heli_obj: codevisual::TextResource,
    pub palm_texture: draw::TextureResource,
}

impl codevisual::Resources for Resources {
    fn new(loader: &codevisual::ResourceLoader) -> Self {
        Self {
            car_texture: draw::Texture::load(loader, "assets/car.png"),
            heli_texture: draw::Texture::load(loader, "assets/heli.png"),
            dirt_texture: draw::Texture::load(loader, "assets/dirt.png"),
            grass_texture: draw::Texture::load(loader, "assets/grass.png"),
            darkgrass_texture: draw::Texture::load(loader, "assets/darkgrass.png"),
            map_texture: draw::Texture::load(loader, "assets/map.png"),
            bush_texture: draw::Texture::load(loader, "assets/bush.png"),
            car_obj: codevisual::load_text(loader, "assets/car.obj"),
            heli_obj: codevisual::load_text(loader, "assets/heli.obj"),
            palm_texture: draw::Texture::load(loader, "assets/palm.png"),
        }
    }
}

impl codevisual::Game for Playground {
    type Resources = Resources;
    fn new(app: Rc<codevisual::Application>, resources: &Resources) -> Self {
        app.set_cursor_type(codevisual::CursorType::Pointer);
        Self {
            app: app.clone(),
            current_time: 0.0,
            camera_position: vec2(0.0, 0.0),
            camera_distance: MAX_CAMERA_DIST / 2.0,
            time_scale: {
                let setting = Rc::new(Cell::new(1.0));
                {
                    let setting = setting.clone();
                    app.add_setting(codevisual::I32Setting {
                                        name: String::from("Time scale"),
                                        min_value: 0,
                                        max_value: 200,
                                        default_value: 100,
                                        setter: move |new_value| {
                                            setting.set(new_value as f32 / 100.0);
                                        },
                                    });
                }
                setting
            },
            start_drag: None,
            prev_zoom_touchdist: 0.0,
            global_uniforms: GlobalUniforms {
                u_time: 0.0,
                u_matrix: Mat4::identity(),
            },
            ground: Ground::new(&app, resources),
            units: Units::new(&app, resources),
        }
    }
    fn update(&mut self, mut delta_time: f32) {
        delta_time *= self.time_scale.get();
        self.current_time += delta_time;
        self.units.update(delta_time);
    }
    fn render<T: draw::Target>(&mut self, target: &mut T) {
        target.clear(Color::rgb(1.0, 1.0, 1.0));
        self.global_uniforms.u_time = self.current_time;
        self.global_uniforms.u_matrix = {
            let (w, h) = self.app.get_size();
            Mat4::perspective(std::f32::consts::PI / 4.0,
                              w as f32 / h as f32,
                              100.0,
                              2500.0) *
            Mat4::translate(vec3(0.0, 0.0, -self.camera_distance)) *
            Mat4::rotate_x(-0.2) *
            Mat4::translate(vec3(self.camera_position.x, self.camera_position.y, 0.0))
        };
        self.units.render(target, &self.global_uniforms);
        self.ground.render(target, &self.global_uniforms);
        self.units.render2(target, &self.global_uniforms);
    }
    fn handle_event(&mut self, event: codevisual::Event) {
        use codevisual::Event::*;
        println!("{:?}", event);
        match event {
            MouseDown {
                x,
                y,
                button: codevisual::MouseButton::Left,
            } => {
                self.app.set_cursor_type(codevisual::CursorType::Drag);
                self.start_drag = Some(vec2(x, y));
            }
            MouseMove { x, y } => {
                if let Some(Vec2 {
                                x: prev_x,
                                y: prev_y,
                            }) = self.start_drag {
                    self.camera_position += vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                                            self.app.get_size().1 as f32 *
                                            self.camera_distance;
                    self.start_drag = Some(vec2(x, y));
                }
            }
            MouseUp { button: codevisual::MouseButton::Left, .. } => {
                self.app.set_cursor_type(codevisual::CursorType::Pointer);
                self.start_drag = None;
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
                                }) = self.start_drag {
                        self.camera_position += vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                                                self.app.get_size().1 as f32 *
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
        self.camera_distance = self.camera_distance
            .min(MAX_CAMERA_DIST)
            .max(MIN_CAMERA_DIST);
    }
}

fn main() {
    codevisual::run::<Playground>();
}