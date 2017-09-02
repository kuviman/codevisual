use ::*;

pub struct Settings {
    pub fog_enabled: codevisual::SettingValue<bool>,
    pub time_scale: codevisual::SettingValue<f64>,
    pub decor_percent: codevisual::SettingValue<f64>,
    pub decor_transparency: codevisual::SettingValue<bool>,
    pub actions_per_tick: codevisual::SettingValue<f64>,
    pub draw_count: codevisual::SettingValue<usize>,
    pub point_updates: codevisual::SettingValue<bool>,
    pub heightmap_enabled: codevisual::SettingValue<bool>,
    pub show_bushes: codevisual::SettingValue<bool>,
    pub clouds_enabled: codevisual::SettingValue<bool>,
}

impl Settings {
    pub fn new(app: &codevisual::Application) -> Self {
        Self {
            decor_percent: app.add_setting_f64("Decor", 0.0, 1.0, 1.0),
            show_bushes: app.add_setting_bool("Bushes", false),
            decor_transparency: app.add_setting_bool("Decor transparency", true),
            clouds_enabled: app.add_setting_bool("Clouds", false),
            fog_enabled: app.add_setting_bool("Fog'o'war", true),
            heightmap_enabled: app.add_setting_bool("Terrait heightmap", true),
            point_updates: app.add_setting_bool("Point updates", false),
            draw_count: app.add_setting_usize("Unit count", 0, units::MAX_COUNT, 100),
            actions_per_tick: app.add_setting_f64("Actions per tick", 0.0, 1.0, 0.2),
            time_scale: app.add_setting_f64("Time scale", 0.0, 4.0, 1.0),
        }
    }
}