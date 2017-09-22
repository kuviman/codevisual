use ::*;

mod ground;
mod weather;
mod trees;

pub use self::ground::Ground;
pub use self::weather::Weather;
pub use self::trees::Trees;

pub struct GameMap {
    app: Rc<codevisual::Application>,
    ground: Ground,
    weather: Weather,
    pub trees: Trees,
    pub size: Vec2<f32>,
}

resources! {
    Resources {
        ground: ground::Resources = (),
        weather: weather::Resources = (),
        trees: trees::Resources = (),
    }
}

impl GameMap {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, settings: &Rc<Settings>, game_log: &game_log::GameLog) -> Self {
        Self {
            app: app.clone(),
            ground: Ground::new(app, resources.ground, game_log, settings),
            weather: Weather::new(app, resources.weather, settings, game_log),
            trees: Trees::new(app, resources.trees, game_log, settings),
            size: vec2(game_log.map_size.x as f32, game_log.map_size.y as f32),
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        self.ground.draw(framebuffer, (&uniforms, self.weather.uniforms()));
        self.trees.draw(framebuffer, &uniforms);
        self.weather.draw(framebuffer, &uniforms);
    }
}