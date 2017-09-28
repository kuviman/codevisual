use ::*;

#[derive(Defines, PartialEq, Clone)]
pub struct ShaderDefines {
    shadows_enabled: bool,
}

pub struct Settings {
    pub time_scale: codevisual::SettingValue<f64>,
    pub sky_height: codevisual::SettingValue<f64>,
    pub draw_minimap: codevisual::SettingValue<bool>,
    pub fov: codevisual::SettingValue<f64>,
    pub clouds_alpha: codevisual::SettingValue<f64>,
    pub clouds_enabled: codevisual::SettingValue<bool>,
    pub draw_attack_rays: codevisual::SettingValue<bool>,
    pub shadows_enabled: codevisual::SettingValue<bool>,
}

impl Settings {
    pub fn new(app: &Rc<codevisual::Application>) -> Rc<Self> {
        Rc::new(Self {
            time_scale: app.add_setting_f64("Time scale", 0.0, 4.0, 1.0),
            sky_height: app.add_setting_f64("Sky height", 0.0, 300.0, 30.0),
            draw_minimap: app.add_setting_bool("Minimap", true),
            draw_attack_rays: app.add_setting_bool("Draw attack rays", false),
            fov: app.add_setting_f64("FOV", 0.1, std::f64::consts::PI / 2.0, std::f64::consts::PI / 3.0),
            clouds_alpha: app.add_setting_f64("Clouds opacity", 0.0, 1.0, 0.3),
            clouds_enabled: app.add_setting_bool("Clouds", true),
            shadows_enabled: app.add_setting_bool("Shadows", true),
        })
    }
    pub fn get_shader_defines(&self) -> ShaderDefines {
        ShaderDefines {
            shadows_enabled: self.shadows_enabled.get(),
        }
    }
}