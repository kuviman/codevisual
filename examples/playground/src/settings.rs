use ::*;

#[derive(Settings)]
pub struct Settings {
    #[setting(name = "Fog'o'war", default = "true")]
    pub fog_enabled: bool,
    #[setting(name = "Time scale", default = "1.0", min = "0.0", max = "4.0")]
    pub time_scale: f64,
    #[setting(min = "0.0", max = "1.0", default = "1.0")]
    pub decor_percent: f64,
    #[setting(default = "true")]
    pub decor_transparency: bool,
    #[setting(min = "0.0", max = "1.0", default = "0.2")]
    pub actions_per_tick: f64,
    #[setting(min = "0", max = "units::MAX_COUNT", default = "100")]
    pub draw_count: usize,
    #[setting(default = "false")]
    pub point_updates: bool,
    #[setting(default = "true")]
    pub heightmap_enabled: bool,
    #[setting(default = "true")]
    pub show_bushes: bool,
    #[setting(name = "Clouds", default = "true")]
    pub clouds_enabled: bool,
}