#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;

#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate brijs;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[cfg(target_os = "emscripten")]
extern crate codewars2017_web;

mod gamelog;

mod camera;

pub ( crate ) use camera::*;

mod terrain;

pub ( crate ) use terrain::*;

struct CodeWars2017 {
    app: Rc<codevisual::Application>,
    camera: Camera,
    terrain: Terrain,
    game_log: gamelog::GameLog,
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
        game_log: gamelog::GameLog = "game.log",
    }
}

impl codevisual::Game for CodeWars2017 {
    type Resources = Resources;

    fn get_title() -> String {
        String::from("CodeWars 2017")
    }

    fn new(app: Rc<codevisual::Application>, resources: Self::Resources) -> Self {
        let app = &app;
        let game_log: gamelog::GameLog = resources.game_log;
        let terrain = Terrain::new(app, &game_log.ticks.read().unwrap()[0]);
        Self {
            app: app.clone(),
            camera: Camera::new(app),
            game_log,
            terrain,
        }
    }

    fn update(&mut self, delta_time: f64) {}

    fn draw(&mut self) {
        let mut framebuffer = ugli::default_framebuffer(self.app.ugli_context());
        let framebuffer = &mut framebuffer;
        ugli::clear(framebuffer, Some(Color::rgb(0.0, 1.0, 1.0)), Some(1.0));
        self.terrain.draw(framebuffer, self.camera.uniforms());
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