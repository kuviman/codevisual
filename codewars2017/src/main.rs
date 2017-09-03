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

pub ( crate ) use gamelog::*;

mod camera;

pub ( crate ) use camera::*;

struct Settings {
    blur_radius: codevisual::SettingValue<i32>,
    blur_sigma: codevisual::SettingValue<f64>,
    blur_div: codevisual::SettingValue<f64>,
    blur: codevisual::SettingValue<bool>,
    view_plain: codevisual::SettingValue<bool>,
}

#[derive(Defines, Default, PartialEq, Clone)]
struct BlurDefines {
    BLUR_RADIUS: i32,
    BLUR_DIV: f32,
    BLUR_SIGMA: f32,
    BLUR: bool,
    VIEW_PLAIN: bool,
}

struct CodeWars2017 {
    app: Rc<codevisual::Application>,
    camera: Camera,
    texture: ugli::Texture2d,
    material: Material<(), BlurDefines>,
    settings: Settings,
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
                app.ugli_context(), (), Default::default(), include_str!("shader.glsl")),
            settings: Settings {
                blur_radius: app.add_setting_i32("Blur Radius", 0, 20, 10),
                blur_sigma: app.add_setting_f64("Blur Sigma", 0.0, 2.0, 1.0),
                blur_div: app.add_setting_f64("Blur Div", 1.0, 16.0, 2.0),
                blur: app.add_setting_bool("Blur", false),
                view_plain: app.add_setting_bool("Plain view", false),
            }
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
        self.material.defines.BLUR_DIV = self.settings.blur_div.get() as f32;
        self.material.defines.BLUR_SIGMA = self.settings.blur_sigma.get() as f32;
        self.material.defines.BLUR_RADIUS = self.settings.blur_radius.get();
        self.material.defines.BLUR = self.settings.blur.get();
        self.material.defines.VIEW_PLAIN = self.settings.view_plain.get();
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