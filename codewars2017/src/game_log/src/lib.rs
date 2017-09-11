#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;

#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate brijs;

#[cfg(target_os = "emscripten")]
extern crate codewars2017_web;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod loader;

pub use loader::Loader;

pub mod raw;

mod vehicle;

pub use vehicle::*;

mod vecmap;

pub use vecmap::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum TerrainType {
    PLAIN,
    SWAMP,
    FOREST,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum WeatherType {
    CLOUD,
    CLEAR,
    RAIN,
}

pub type ID = u32;

type TerrainHolder = Arc<Vec<Vec<TerrainType>>>;
type WeatherHolder = Arc<Vec<Vec<WeatherType>>>;

#[derive(Debug)]
pub struct GameLog {
    pub tick_count: usize,
    pub map_size: Vec2<f32>,
    pub terrain: TerrainHolder,
    pub weather: WeatherHolder,
    pub vehicles: Vehicles,
    pub loaded_tick_count: usize,
}

impl GameLog {
    fn new(mut tick0: raw::TickInfo) -> Self {
        let terrain = {
            let mut terrain = None;
            std::mem::swap(&mut tick0.terrainByCellXY, &mut terrain);
            TerrainHolder::new(terrain.unwrap())
        };
        let weather = {
            let mut weather = None;
            std::mem::swap(&mut tick0.weatherByCellXY, &mut weather);
            WeatherHolder::new(weather.unwrap())
        };
        let mut game_log = Self {
            terrain: terrain.clone(),
            weather: weather.clone(),
            tick_count: tick0.tickCount.unwrap(),
            vehicles: Vehicles::new(&terrain, &weather),
            map_size: vec2(tick0.width.unwrap() as f32, tick0.height.unwrap() as f32),
            loaded_tick_count: 0,
        };
        game_log.add_tick(tick0);
        game_log
    }
    fn add_tick(&mut self, tick_info: raw::TickInfo) {
        self.loaded_tick_count += 1;
        let tick = tick_info.tickIndex;
        self.vehicles.add_tick(tick, tick_info.vehicles, tick_info.decoratedVehicleById, &tick_info.effects);
    }
    fn finish(&mut self) {}
}