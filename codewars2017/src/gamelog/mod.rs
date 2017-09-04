use ::*;

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

#[derive(Debug)]
pub struct GameLog {
    pub tick_count: usize,
    pub terrain: Vec<Vec<TerrainType>>,
}

impl GameLog {
    fn new(mut tick0: loader::raw::TickInfo) -> Self {
        let mut terrain = None;
        std::mem::swap(&mut tick0.terrainByCellXY, &mut terrain);
        let mut game_log = Self {
            terrain: terrain.unwrap(),
            tick_count: tick0.tickCount.unwrap(),
        };
        game_log.add_tick(tick0);
        game_log
    }
    fn add_tick(&mut self, tick_info: loader::raw::TickInfo) {}
    fn finish(&mut self) {}
}