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

pub mod loader;

#[derive(Serialize, Deserialize, Debug)]
pub enum TerrainType {
    PLAIN,
    SWAMP,
    FOREST,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WeatherType {
    CLOUD,
    CLEAR,
    RAIN,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VehicleType {
    ARRV,
    IFV,
    TANK,
    HELICOPTER,
    FIGHTER,
}

pub type ID = u32;

#[derive(Debug)]
pub struct GameLog {
    pub tick_count: usize,
    pub terrain: Vec<Vec<TerrainType>>,
}

impl GameLog {
    fn new(mut tick0: loader::TickInfo) -> Self {
        let mut terrain = None;
        std::mem::swap(&mut tick0.terrainByCellXY, &mut terrain);
        let mut game_log = Self {
            terrain: terrain.unwrap(),
            tick_count: tick0.tickCount.unwrap(),
        };
        game_log.add_tick(tick0);
        game_log
    }
    fn add_tick(&mut self, tick_info: loader::TickInfo) {}
    fn finish(&mut self) {}
}