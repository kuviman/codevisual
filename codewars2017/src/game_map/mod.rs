use ::*;

mod ground;
mod weather;

pub use self::ground::Ground;
pub use self::weather::Weather;

pub struct GameMap {
    app: Rc<codevisual::Application>,
    ground: Ground,
    weather: Weather,
    pub size: Vec2<f32>,
}

resources! {
    Resources {
        ground: ground::Resources = (),
        weather: weather::Resources = (),
    }
}

impl GameMap {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, game_log: &game_log::GameLog) -> Self {
        Self {
            app: app.clone(),
            ground: Ground::new(app, resources.ground, game_log),
            weather: Weather::new(app, resources.weather, game_log),
            size: vec2(game_log.map_size.x as f32, game_log.map_size.y as f32),
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::DefaultFramebuffer, uniforms: U) {
        self.ground.draw(framebuffer, (&uniforms, self.weather.uniforms()));
        self.weather.draw(framebuffer, &uniforms);
    }
}