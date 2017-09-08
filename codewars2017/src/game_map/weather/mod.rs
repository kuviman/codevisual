use ::*;

mod clouds;

use self::clouds::*;

mod rain;

use self::rain::*;

pub type Resources = ();

pub struct Weather {
    clouds: Clouds,
    clouds_enabled: codevisual::SettingValue<bool>,
    rain: Rain,
    weather_map: ugli::Texture2d,
}

impl Weather {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, game_log: &GameLog) -> Self {
        Self {
            clouds: Clouds::new(app, game_log),
            rain: Rain::new(app, game_log),
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
            clouds_enabled: app.add_setting_bool("Clouds", true),
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::DefaultFramebuffer, uniforms: U) {
        let uniforms = (uniforms, uniforms!(weather_map: &self.weather_map));
        self.rain.draw(framebuffer, &uniforms);
        if self.clouds_enabled.get() {
            self.clouds.draw(framebuffer, &uniforms);
        }
    }
    pub fn uniforms(&self) -> ugli::SingleUniform<&ugli::Texture2d> {
        uniforms!(weather_map: &self.weather_map)
    }
}