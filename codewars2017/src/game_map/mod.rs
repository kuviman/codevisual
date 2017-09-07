use ::*;

#[derive(Vertex)]
struct RainVertex {
    a_v: Vec3<f32>
}

pub struct GameMap {
    app: Rc<codevisual::Application>,
    texture: ugli::Texture2d,
    texture_weather: ugli::Texture2d,
    material: Material<(), BlurDefines>,
    material_weather: Material<(), BlurDefines>,
    material_rain: Material<(), ()>,
    pub size: Vec2<f32>,
    settings: Settings,
    sky_enabled: codevisual::SettingValue<bool>,
    rain_mesh: ugli::VertexBuffer<RainVertex>,
    resources: Resources,
}

resources! {
    Resources {
        plain_texture: ugli::Texture2d = "assets/grass.png",
        forest_texture: ugli::Texture2d = "assets/darkgrass.png",
        swamp_texture: ugli::Texture2d = "assets/dirt.png",
    }
}

impl GameMap {
    pub fn new(app: &Rc<codevisual::Application>, mut resources: Resources, game_log: &game_log::GameLog) -> Self {
        resources.plain_texture.set_wrap_mode(ugli::WrapMode::Repeat);
        resources.forest_texture.set_wrap_mode(ugli::WrapMode::Repeat);
        resources.swamp_texture.set_wrap_mode(ugli::WrapMode::Repeat);
        Self {
            app: app.clone(),
            resources,
            texture: {
                let terrain_data: &Vec<Vec<game_log::TerrainType>> = &game_log.terrain;
                let mut texture = ugli::Texture2d::new_with(app.ugli_context(), vec2(terrain_data.len(), terrain_data[0].len()), |pos| {
                    use game_log::TerrainType::*;
                    match terrain_data[pos.x][pos.y] {
                        PLAIN => Color::rgb(1.0, 0.0, 0.0),
                        FOREST => Color::rgb(0.0, 1.0, 0.0),
                        SWAMP => Color::rgb(0.0, 0.0, 1.0),
                    }
                });
                //                texture.set_filter(ugli::Filter::Nearest);
                texture
            },
            texture_weather: {
                let weather_data: &Vec<Vec<game_log::WeatherType>> = &game_log.weather;
                let mut texture = ugli::Texture2d::new_with(app.ugli_context(), vec2(weather_data.len(), weather_data[0].len()), |pos| {
                    use game_log::WeatherType::*;
                    match weather_data[pos.x][pos.y] {
                        CLEAR => Color::rgb(1.0, 0.0, 0.0),
                        CLOUD => Color::rgb(0.0, 1.0, 0.0),
                        RAIN => Color::rgb(0.0, 0.0, 1.0),
                    }
                });
                //                texture.set_filter(ugli::Filter::Nearest);
                texture
            },
            size: vec2(game_log.map_size.x as f32, game_log.map_size.y as f32),
            material: Material::new(
                app.ugli_context(), (), Default::default(), include_str!("shader.glsl")),
            material_weather: Material::new(
                app.ugli_context(), (), Default::default(), include_str!("shader_weather.glsl")),
            material_rain: Material::new(
                app.ugli_context(), (), (), include_str!("rain.glsl")),
            settings: Settings {
                blur_radius: app.add_setting_i32("Blur Radius", 0, 20, 1),
                blur_sigma: app.add_setting_f64("Blur Sigma", 0.0, 2.0, 2.0),
                blur_div: app.add_setting_f64("Blur Div", 1.0, 16.0, 2.0),
                blur: app.add_setting_bool("Blur", true),
                view_plain: app.add_setting_bool("Plain view", true),
            },
            sky_enabled: app.add_setting_bool("Clouds", true),
            rain_mesh: ugli::VertexBuffer::new_static(app.ugli_context(), {
                let mut vs = Vec::new();
                let weather_data: &Vec<Vec<game_log::WeatherType>> = &game_log.weather;
                for i in 0..weather_data.len() as usize {
                    for j in 0..weather_data[0].len() as usize {
                        if let game_log::WeatherType::RAIN = weather_data[i][j] {
                            const CELL_SIZE: f32 = 32.0;

                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 1.0) });

                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 1.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE, 1.0) });

                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 1.0) });

                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 1.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE, 1.0) });
                        }
                    }
                }
                vs
            }),
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::DefaultFramebuffer, uniforms: U) {
        self.material.defines.BLUR_DIV = self.settings.blur_div.get() as f32;
        self.material.defines.BLUR_SIGMA = self.settings.blur_sigma.get() as f32;
        self.material.defines.BLUR_RADIUS = self.settings.blur_radius.get();
        self.material.defines.BLUR = self.settings.blur.get();
        self.material.defines.VIEW_PLAIN = self.settings.view_plain.get();
        self.material_weather.defines = self.material.defines.clone();
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::TriangleFan,
            &ugli::plain(&ugli::quad(self.app.ugli_context()).slice(..)),
            (&uniforms, uniforms! {
                texture: &self.texture,
                plain_texture: &self.resources.plain_texture,
                forest_texture: &self.resources.forest_texture,
                swamp_texture: &self.resources.swamp_texture,
                map_size: &self.size,
            }),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::On,
                blend_mode: ugli::BlendMode::Off,
                ..Default::default()
            }
        );
        if self.sky_enabled.get() {
            ugli::draw(
                framebuffer,
                &self.material_rain.ugli_program(),
                ugli::DrawMode::Triangles,
                &ugli::plain(&self.rain_mesh.slice(..)),
                &uniforms,
                &ugli::DrawParameters {
                    depth_test: ugli::DepthTest::Off,
                    blend_mode: ugli::BlendMode::Alpha,
                    ..Default::default()
                }
            );
            ugli::draw(
                framebuffer,
                &self.material_weather.ugli_program(),
                ugli::DrawMode::TriangleFan,
                &ugli::plain(&ugli::quad(self.app.ugli_context()).slice(..)),
                (&uniforms, uniforms! {
                    texture: &self.texture_weather,
                    map_size: &self.size,
                }),
                &ugli::DrawParameters {
                    depth_test: ugli::DepthTest::On,
                    blend_mode: ugli::BlendMode::Alpha,
                    ..Default::default()
                }
            );
        }
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