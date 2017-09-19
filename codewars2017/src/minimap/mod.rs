use ::*;

#[derive(Vertex, Clone)]
struct BoundVertex {
    a_pos: Vec2<f32>,
}

pub struct Minimap {
    app: Rc<codevisual::Application>,
    map: ugli::Texture2d,
    vehicle_material: Material,
    background_material: Material,
    bound_material: Material,
    bound_geometry: ugli::VertexBuffer<BoundVertex>,
}

impl Minimap {
    pub fn new(app: &Rc<codevisual::Application>, game_log: &GameLog) -> Self {
        Self {
            app: app.clone(),
            vehicle_material: Material::new(app.ugli_context(), (), (), include_str!("vehicle.glsl")),
            background_material: Material::new(app.ugli_context(), (), (), include_str!("background.glsl")),
            bound_material: Material::new(app.ugli_context(), (), (), include_str!("bound.glsl")),
            bound_geometry: ugli::VertexBuffer::new_dynamic(app.ugli_context(), vec![BoundVertex { a_pos: vec2(0.0, 0.0) }; 4]),
            map: {
                let mut map = ugli::Texture2d::new_with(
                    app.ugli_context(),
                    vec2(game_log.terrain.len(), game_log.terrain[0].len()),
                    |pos| {
                        use game_log::TerrainType::*;
                        let terrain_color = match game_log.terrain[pos.x][pos.y] {
                            PLAIN => Color::rgb(0.5, 1.0, 0.5),
                            FOREST => Color::rgb(0.0, 1.0, 0.0),
                            SWAMP => Color::rgb(0.0, 0.5, 0.5),
                        };
                        use game_log::WeatherType::*;
                        let weather_color = match game_log.weather[pos.x][pos.y] {
                            CLEAR => Color::rgba(0.0, 0.0, 0.0, 0.0),
                            CLOUD => Color::rgba(1.0, 1.0, 1.0, 0.3),
                            RAIN => Color::rgba(0.5, 0.5, 0.7, 0.3),
                        };
                        Color::mix(terrain_color, weather_color, 1.0 - weather_color.alpha, weather_color.alpha)
                    });
                map.set_filter(ugli::Filter::Nearest);
                map
            },
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&mut self,
                                         vehicles: &vehicles::Vehicles,
                                         map: &game_map::GameMap,
                                         camera: &Camera,
                                         framebuffer: &mut ugli::Framebuffer,
                                         uniforms: U) {
        #[cfg(target_os = "emscripten")]
        let offset = codewars2017_web::get_controls_offset() * 2.0;
        #[cfg(not(target_os = "emscripten"))]
        let offset = 0.0;

        let matrix: Mat4<f32> = Mat4::translate(vec3(-0.7, -0.6 + offset, 0.0)) *
            Mat4::scale(vec3(0.2 * framebuffer.get_size().y as f32 / framebuffer.get_size().x as f32, 0.2, 0.2)) *
            Mat4::rotate_z(camera.rotation) *
            Mat4::scale(vec3(1.0, -1.0, 1.0));
        let uniforms = (
            uniforms! {
                minimatrix: matrix,
                map_size: map.size,
            }, uniforms);
        ugli::draw(
            framebuffer,
            &self.background_material.ugli_program(),
            ugli::Quad::DRAW_MODE,
            &ugli::plain(&ugli::quad(self.app.ugli_context()).slice(..)),
            (&uniforms, uniforms!(map: &self.map)),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                blend_mode: ugli::BlendMode::Alpha,
                ..Default::default()
            }
        );
        for instances in vehicles.get_instances() {
            ugli::draw(
                framebuffer,
                &self.vehicle_material.ugli_program(),
                ugli::DrawMode::Points,
                &ugli::plain(&instances),
                &uniforms,
                &ugli::DrawParameters {
                    depth_test: ugli::DepthTest::Off,
                    blend_mode: ugli::BlendMode::Off,
                    ..Default::default()
                });
        }
        let bound_vs = [
            vec2(-1.0, -1.0),
            vec2(1.0, -1.0),
            vec2(1.0, 1.0),
            vec2(-1.0, 1.0)];
        for (out, in_pos) in self.bound_geometry.slice_mut(..).iter_mut().zip(bound_vs.into_iter()) {
            out.a_pos = camera.raytrace(*in_pos);
        }
        ugli::draw(
            framebuffer,
            &self.bound_material.ugli_program(),
            ugli::DrawMode::LineLoop,
            &ugli::plain(&self.bound_geometry.slice(..)),
            &uniforms,
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                blend_mode: ugli::BlendMode::Alpha,
                ..Default::default()
            }
        );
    }
}