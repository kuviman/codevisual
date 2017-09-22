use ::*;

mod clouds;

use self::clouds::*;

mod rain;

use self::rain::*;

pub type Resources = ();

pub struct Weather {
    clouds: Clouds,
    settings: Rc<Settings>,
    rain: Rain,
    weather_map: ugli::Texture2d,
}

impl Weather {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, settings: &Rc<Settings>, game_log: &GameLog) -> Self {
        Self {
            clouds: Clouds::new(app, settings, game_log),
            rain: Rain::new(app, game_log, settings),
            weather_map: {
                let weather_data: &Vec<Vec<game_log::WeatherType>> = &game_log.weather;
                blur::gauss(app.ugli_context(), &ugli::Texture2d::new_with(
                    app.ugli_context(),
                    vec2(weather_data.len(), weather_data[0].len()),
                    |pos| {
                        use game_log::WeatherType::*;
                        match weather_data[pos.x][pos.y] {
                            CLEAR => Color::rgb(1.0, 0.0, 0.0),
                            CLOUD => Color::rgb(0.0, 1.0, 0.0),
                            RAIN => Color::rgb(0.0, 0.0, 1.0),
                        }
                    }))
            },
            settings: settings.clone(),
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        let uniforms = (uniforms, uniforms!(weather_map: &self.weather_map));
        self.rain.draw(framebuffer, &uniforms);
        if self.settings.clouds_enabled.get() {
            self.clouds.draw(framebuffer, &uniforms);
        }
    }
    pub fn uniforms(&self) -> ugli::SingleUniform<&ugli::Texture2d> {
        uniforms!(weather_map: &self.weather_map)
    }
}