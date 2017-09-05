use ::*;

pub struct Terrain {
    app: Rc<codevisual::Application>,
    texture: ugli::Texture2d,
    material: Material<(), BlurDefines>,
    pub size: Vec2<f32>,
    settings: Settings,
    resources: Resources,
}

resources! {
    Resources {
        plain_texture: ugli::Texture2d = "assets/grass.png",
        forest_texture: ugli::Texture2d = "assets/darkgrass.png",
        swamp_texture: ugli::Texture2d = "assets/dirt.png",
    }
}

impl Terrain {
    pub fn new(app: &Rc<codevisual::Application>, mut resources: Resources, game_log: &gamelog::GameLog) -> Self {
        resources.plain_texture.set_wrap_mode(ugli::WrapMode::Repeat);
        resources.forest_texture.set_wrap_mode(ugli::WrapMode::Repeat);
        resources.swamp_texture.set_wrap_mode(ugli::WrapMode::Repeat);
        Self {
            app: app.clone(),
            resources,
            texture: {
                let terrain_data: &Vec<Vec<gamelog::TerrainType>> = &game_log.terrain;
                ugli::Texture2d::new_with(app.ugli_context(), vec2(terrain_data.len(), terrain_data[0].len()), |pos| {
                    use gamelog::TerrainType::*;
                    match terrain_data[pos.x][pos.y] {
                        PLAIN => Color::rgb(1.0, 0.0, 0.0),
                        FOREST => Color::rgb(0.0, 1.0, 0.0),
                        SWAMP => Color::rgb(0.0, 0.0, 1.0),
                    }
                })
            },
            size: vec2(game_log.map_size.x as f32, game_log.map_size.y as f32),
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

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::DefaultFramebuffer, uniforms: U) {
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
            (uniforms, uniforms! {
                texture: &self.texture,
                plain_texture: &self.resources.plain_texture,
                forest_texture: &self.resources.forest_texture,
                swamp_texture: &self.resources.swamp_texture,
                map_size: &self.size,
            }),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                blend_mode: ugli::BlendMode::Off,
                ..Default::default()
            }
        );
    }
}

struct Settings {
    blur_radius: codevisual::SettingValue<i32>,
    blur_sigma: codevisual::SettingValue<f64>,
    blur_div: codevisual::SettingValue<f64>,
    blur: codevisual::SettingValue<bool>,
    view_plain: codevisual::SettingValue<bool>,
}

#[allow(non_snake_case)]
#[derive(Defines, Default, PartialEq, Clone)]
struct BlurDefines {
    BLUR_RADIUS: i32,
    BLUR_DIV: f32,
    BLUR_SIGMA: f32,
    BLUR: bool,
    VIEW_PLAIN: bool,
}