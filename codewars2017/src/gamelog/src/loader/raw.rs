use ::*;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    id: ID,
    x: Option<f32>,
    y: Option<f32>,
    radius: Option<f32>,
    playerId: Option<ID>,
    durability: Option<i32>,
    maxDurability: Option<i32>,
    maxSpeed: Option<f32>,
    visionRange: Option<i32>,
    squaredVisionRange: Option<i32>,
    groundAttackRange: Option<i32>,
    squaredGroundAttackRange: Option<i32>,
    aerialAttackRange: Option<i32>,
    squaredAerialAttackRange: Option<i32>,
    groundDamage: Option<i32>,
    aerialDamage: Option<i32>,
    groundDefence: Option<i32>,
    aerialDefence: Option<i32>,
    attackCooldownTicks: Option<i32>,
    #[serde(rename = "type")]
    typ: Option<VehicleType>,
    aerial: Option<bool>,
    selected: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TickInfo {
    pub tickIndex: usize,
    pub tickCount: Option<usize>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub randomSeed: Option<i64>,
    pub terrainByCellXY: Option<Vec<Vec<TerrainType>>>,
    pub weatherByCellXY: Option<Vec<Vec<WeatherType>>>,
    pub vehicles: Option<Vec<Vehicle>>,
}