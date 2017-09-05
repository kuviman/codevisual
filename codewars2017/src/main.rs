#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;

#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate brijs;

#[cfg(target_os = "emscripten")]
extern crate codewars2017_web;

extern crate codewars2017_log as gamelog;

mod camera;

pub ( crate ) use camera::*;

mod terrain;

use terrain::Terrain;

mod vehicles;

use vehicles::Vehicles;

struct CodeWars2017 {
    app: Rc<codevisual::Application>,
    camera: Camera,
    terrain: Terrain,
    vehicles: Vehicles,
    game_log_loader: gamelog::loader::Loader,
    current_time: Rc<Cell<f32>>,
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
        game_log_loader: gamelog::loader::Loader = "game.log",
        terrain: terrain::Resources = (),
    }
}

impl codevisual::Game for CodeWars2017 {
    type Resources = Resources;

    fn get_title() -> String {
        String::from("CodeWars 2017")
    }

    fn new(app: &Rc<codevisual::Application>, resources: Self::Resources) -> Self {
        let game_log_loader: gamelog::loader::Loader = resources.game_log_loader;
        let terrain = Terrain::new(app, resources.terrain, &game_log_loader.read());
        let vehicles = Vehicles::new(app, &game_log_loader);
        let mut camera = Camera::new(app);
        camera.position = (terrain.size / 2.0).extend(0.0);
        let current_time = Rc::new(Cell::new(0.0));
        #[cfg(target_os = "emscripten")]
        {
            let current_time = current_time.clone();
            let game_log_loader = game_log_loader.clone();
            run_js! {
                CodeWars.init(brijs::Callback::from(move |pos: i32| {
                    current_time.set(pos as f32 / 1000.0 * game_log_loader.read().tick_count as f32 / 60.0);
                }));
            }
        }
        Self {
            app: app.clone(),
            camera,
            game_log_loader,
            terrain,
            vehicles,
            current_time,
        }
    }

    fn update(&mut self, delta_time: f64) {
        let new_time = f32::min(
            self.current_time.get() + delta_time as f32,
            self.game_log_loader.read().loaded_tick_count as f32 / 60.0);
        self.current_time.set(f32::max(self.current_time.get(), new_time));
        #[cfg(target_os = "emscripten")]
        run_js! {
            CodeWars.set_playback_position(
                &((self.current_time.get() * 60.0) as usize),
                &self.game_log_loader.read().tick_count);
        };
    }

    fn draw(&mut self) {
        let tick = min((self.current_time.get() * 60.0) as usize, self.game_log_loader.read().loaded_tick_count - 1);
        let mut framebuffer = ugli::default_framebuffer(self.app.ugli_context());
        let framebuffer = &mut framebuffer;
        ugli::clear(framebuffer, Some(Color::rgb(0.0, 1.0, 1.0)), Some(1.0));
        let uniforms = self.camera.uniforms();
        self.terrain.draw(framebuffer, &uniforms);
        self.vehicles.draw(tick, framebuffer, &uniforms);
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