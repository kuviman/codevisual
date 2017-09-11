#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;

#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate brijs;

#[cfg(target_os = "emscripten")]
extern crate codewars2017_web;

extern crate codewars2017_log as game_log;

pub ( crate ) use game_log::GameLog;

mod camera;

pub ( crate ) use camera::*;

mod game_map;

use game_map::GameMap;

mod vehicles;

use vehicles::Vehicles;

mod blur;

mod skybox;

use skybox::SkyBox;

struct CodeWars2017 {
    app: Rc<codevisual::Application>,
    paused: Rc<Cell<bool>>,
    camera: Camera,
    skybox: SkyBox,
    terrain: GameMap,
    vehicles: Vehicles,
    game_log_loader: game_log::Loader,
    current_time: Rc<Cell<f32>>,
    time_scale: codevisual::SettingValue<f64>,
    sky_height: codevisual::SettingValue<f64>,
}

shader_library! {
    ShaderLib {
        "codewars" => include_str!("lib.glsl"),
        "camera" => include_str!("camera/lib.glsl"),
    }
}

type Material<U = (), D = ()> = codevisual::Material<ShaderLib, U, D>;

resources! {
    Resources {
        game_log_loader: game_log::Loader = "game.log",
        skybox: skybox::Resources = (),
        terrain: game_map::Resources = (),
    }
}

impl codevisual::Game for CodeWars2017 {
    type Resources = Resources;

    fn get_title() -> String {
        String::from("CodeWars 2017")
    }

    fn new(app: &Rc<codevisual::Application>, resources: Self::Resources) -> Self {
        let game_log_loader: game_log::Loader = resources.game_log_loader;
        let terrain = GameMap::new(app, resources.terrain, &game_log_loader.read());
        let vehicles = Vehicles::new(app, &game_log_loader);
        let camera = Camera::new(app, terrain.size);
        let current_time = Rc::new(Cell::new(0.0));
        let paused = Rc::new(Cell::new(false));
        #[cfg(target_os = "emscripten")]
        {
            let current_time = current_time.clone();
            let game_log_loader = game_log_loader.clone();
            run_js! {
                CodeWars.set_timeline_callback(brijs::Callback::from(move |pos: i32| {
                    current_time.set(pos as f32 / 1000.0 * game_log_loader.read().tick_count as f32 / 60.0);
                }));
            }
        }
        #[cfg(target_os = "emscripten")]
        {
            let paused = paused.clone();
            run_js! {
                CodeWars.set_paused(&paused.get());
                CodeWars.set_play_button_callback(brijs::Callback::from(move |_: ()| {
                    paused.set(!paused.get());
                    run_js!{ CodeWars.set_paused(&paused.get()); };
                }));
            }
        }
        Self {
            app: app.clone(),
            paused,
            skybox: SkyBox::new(app, resources.skybox),
            camera,
            game_log_loader,
            terrain,
            vehicles,
            current_time,
            time_scale: app.add_setting_f64("Time scale", 0.0, 4.0, 1.0),
            sky_height: app.add_setting_f64("Sky height", 30.0, 300.0, 60.0),
        }
    }

    fn update(&mut self, delta_time: f64) {
        if !self.paused.get() {
            let delta_time = delta_time * self.time_scale.get();
            let new_time = f32::min(
                self.current_time.get() + delta_time as f32,
                self.game_log_loader.read().loaded_tick_count as f32 / 60.0);
            self.current_time.set(f32::max(self.current_time.get(), new_time));
        }
        #[cfg(target_os = "emscripten")]
        run_js! {
            CodeWars.set_playback_position(
                &((self.current_time.get() * 60.0) as usize),
                &self.game_log_loader.read().tick_count);
        };
    }

    fn draw(&mut self) {
        let tick = (self.current_time.get() * 60.0) as usize;
        let max_tick = self.game_log_loader.read().loaded_tick_count - 1;
        if !self.paused.get() && tick <= max_tick {
            let mut framebuffer = self.app.ugli_context().default_framebuffer();
            let framebuffer = &mut framebuffer;
            let uniforms = (
                uniforms! {
                    u_sky_height: self.sky_height.get() as f32,
                    u_current_time: self.current_time.get() as f32,
                    u_cell_size: 32.0, // TODO
                },
                self.camera.uniforms());
            self.skybox.draw(framebuffer, &uniforms);
            ugli::clear(framebuffer, None, Some(1.0));
            self.vehicles.draw(tick, framebuffer, &uniforms);
            self.terrain.draw(framebuffer, &uniforms);
        }
    }

    fn handle_event(&mut self, event: codevisual::Event) {
        self.camera.handle(event);
    }
}

fn main() {
    #[cfg(target_os = "emscripten")]
    brijs::run_script(codewars2017_web::JS_SOURCE);
    #[cfg(not(target_os = "emscripten"))]
    std::env::set_current_dir("static").unwrap();
    codevisual::run::<CodeWars2017>()
}