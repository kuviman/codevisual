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
const MIN_CAMERA_DIST: f32 = 6.0;
const MAX_CAMERA_DIST: f32 = 2000.0;

#[derive(Uniforms)]
pub struct GlobalUniforms {
    u_time: f32,
    u_matrix: Mat4<f32>,
}

struct Settings {
    actions_per_tick: usize,
    time_scale: f32,
    draw_count: usize,
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
    settings: Rc<RefCell<Settings>>,
}

impl Playground {
    fn create_settings(app: &codevisual::Application) -> Rc<RefCell<Settings>> {
        let settings = Rc::new(RefCell::new(Settings {
                                                time_scale: 1.0,
                                                actions_per_tick: 1000,
                                                draw_count: units::MAX_COUNT,
                                            }));
        {
            let settings = settings.clone();
            app.add_setting(codevisual::I32Setting {
                                name: String::from("Count"),
                                min_value: 1,
                                max_value: units::MAX_COUNT as i32,
                                default_value: {
                                    let borrow = settings.borrow();
                                    borrow.draw_count as i32
                                },
                                setter: move |new_value| {
                                    println!("Drawing {} instances", new_value);
                                    settings.borrow_mut().draw_count = new_value as usize;
                                },
                            });
        }
        {
            let settings = settings.clone();
            app.add_setting(codevisual::I32Setting {
                                name: String::from("Actions per tick"),
                                min_value: 0,
                                max_value: 1000,
                                default_value: {
                                    let borrow = settings.borrow();
                                    borrow.actions_per_tick as i32
                                },
                                setter: move |new_value| {
                                    settings.borrow_mut().actions_per_tick = new_value as usize;
                                },
                            });
        }
        {
            let settings = settings.clone();
            app.add_setting(codevisual::I32Setting {
                                name: String::from("Time scale"),
                                min_value: 0,
                                max_value: 200,
                                default_value: 100,
                                setter: move |new_value| {
                                    settings.borrow_mut().time_scale = new_value as f32 / 100.0;
                                },
                            });
        }
        settings
    }
}

impl codevisual::Game for Playground {
    fn new(app: Rc<codevisual::Application>) -> Self {
        app.set_cursor_type(codevisual::CursorType::Pointer);
        Self {
            app: app.clone(),
            current_time: 0.0,
            camera_position: vec2(0.0, 0.0),
            camera_distance: MAX_CAMERA_DIST / 2.0,
            settings: Self::create_settings(&app),
            start_drag: None,
            prev_zoom_touchdist: 0.0,
            global_uniforms: GlobalUniforms {
                u_time: 0.0,
                u_matrix: Mat4::identity(),
            },
            ground: Ground::new(&app),
            units: Units::new(&app),
        }
    }
    fn update(&mut self, mut delta_time: f32) {
        delta_time *= self.settings.borrow().time_scale;
        self.current_time += delta_time;
        self.units.actions_per_tick = self.settings.borrow().actions_per_tick;
        self.units.update(delta_time);
    }
    fn render<T: draw::Target>(&mut self, target: &mut T) {
        target.clear(Color::rgb(1.0, 1.0, 1.0));
        self.global_uniforms.u_time = self.current_time;
        self.global_uniforms.u_matrix = {
            let (w, h) = self.app.get_size();
            Mat4::perspective(std::f32::consts::PI / 4.0,
                              w as f32 / h as f32,
                              1.0,
                              100000.0) *
            Mat4::translate(vec3(self.camera_position.x,
                                 self.camera_position.y,
                                 -self.camera_distance))
        };
        self.ground.render(target, &self.global_uniforms);
        self.units
            .render(self.settings.borrow().draw_count,
                    target,
                    &self.global_uniforms);
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
                self.camera_distance = (self.camera_distance * f32::exp(delta as f32 / 1000.0))
                    .min(MAX_CAMERA_DIST)
                    .max(MIN_CAMERA_DIST)
            }
            _ => (),
        }
    }
}

fn main() {
    codevisual::run::<Playground>();
}