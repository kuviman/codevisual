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

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderType {
    MOVE,
    ROTATE,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub action: OrderType,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub maxSpeed: f32,
    pub maxAngularSpeed: f32,
    pub initialX: f32,
    pub initialY: f32,
    pub targetX: f32,
    pub targetY: f32,
    pub gameTime: usize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DecoratedVehicle {
    pub orderExecuted: Option<bool>,
    pub order: Option<Order>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum EffectType {
    VEHICLE_DEATH,
    VEHICLE_CONDITION_CHANGE,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Effect {
    pub id: ID,
    #[serde(rename = "type")]
    pub typ: EffectType,
    pub tick: usize,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub attributes: serde_json::Map<String, serde_json::Value>,
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
    pub decoratedVehicleById: Option<HashMap<String, DecoratedVehicle>>,
    pub effects: Option<Vec<Effect>>,
}