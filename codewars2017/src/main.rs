#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;
#[cfg(target_os = "emscripten")]
pub ( crate ) use codevisual::brijs;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[cfg(target_os = "emscripten")]
extern crate codewars2017_web;

mod gamelog;

pub ( crate ) use gamelog::*;

mod camera;

pub ( crate ) use camera::*;

struct CodeWars2017 {
    app: Rc<codevisual::Application>,
    camera: Camera,
    texture: ugli::Texture2d,
    material: Material,
}

shader_library! {
    ShaderLib {
        "codewars" => include_str!("lib.glsl"),
        "camera" => include_str!("camera/lib.glsl"),
    }
}

type Material = codevisual::Material<ShaderLib>;

resources! {
    Resources {
        game_log: GameLog = "game.log",
    }
}

impl codevisual::Game for CodeWars2017 {
    type Resources = Resources;

    fn get_title() -> String {
        String::from("CodeWars 2017")
    }

    fn new(app: Rc<codevisual::Application>, resources: Self::Resources) -> Self {
        Self {
            app: app.clone(),
            camera: Camera::new(),
            texture: {
                let ticks = resources.game_log.ticks.read().unwrap();
                let terrain_data: &Vec<Vec<TerrainType>> = ticks[0].terrainByCellXY.as_ref().unwrap();
                ugli::Texture2d::new_with(app.ugli_context(), vec2(terrain_data.len(), terrain_data[0].len()), |pos| {
                    match terrain_data[pos.x][pos.y] {
                        TerrainType::PLAIN => Color::rgb(1.0, 0.0, 0.0),
                        TerrainType::FOREST => Color::rgb(0.0, 1.0, 0.0),
                        TerrainType::SWAMP => Color::rgb(0.0, 0.0, 1.0),
                    }
                })
            },
            material: Material::new(
                app.ugli_context(), (), (), include_str!("shader.glsl"))
        }
    }

    fn update(&mut self, delta_time: f64) {}

    fn draw(&mut self) {
        let mut framebuffer = ugli::default_framebuffer(self.app.ugli_context());
        let framebuffer = &mut framebuffer;
        ugli::clear(framebuffer, Some(Color::rgb(0.0, 1.0, 1.0)), Some(1.0));
        self.camera.update({
            let size = self.app.window().get_size();
            vec2(size.x as f32, size.y as f32)
        });
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::TriangleFan,
            &ugli::plain(&ugli::quad(self.app.ugli_context()).slice(..)),
            (self.camera.uniforms(), uniforms!(texture: &self.texture)),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                blend_mode: ugli::BlendMode::Off,
                ..Default::default()
            }
        );
    }

    fn handle_event(&mut self, event: codevisual::Event) {}
}

fn main() {
    #[cfg(target_os = "emscripten")]
    brijs::run_script(codewars2017_web::JS_SOURCE);
    #[cfg(not(target_os = "emscripten"))]
    std::env::set_current_dir("static").unwrap();
    codevisual::run::<CodeWars2017>()
}