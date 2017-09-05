use ::*;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    pub id: ID,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub radius: Option<f32>,
    pub playerId: Option<ID>,
    pub durability: Option<i32>,
    pub maxDurability: Option<i32>,
    pub maxSpeed: Option<f32>,
    pub visionRange: Option<i32>,
    pub squaredVisionRange: Option<i32>,
    pub groundAttackRange: Option<i32>,
    pub squaredGroundAttackRange: Option<i32>,
    pub aerialAttackRange: Option<i32>,
    pub squaredAerialAttackRange: Option<i32>,
    pub groundDamage: Option<i32>,
    pub aerialDamage: Option<i32>,
    pub groundDefence: Option<i32>,
    pub aerialDefence: Option<i32>,
    pub attackCooldownTicks: Option<i32>,
    #[serde(rename = "type")]
    pub typ: Option<VehicleType>,
    pub aerial: Option<bool>,
    pub selected: Option<bool>,
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