use ::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum VehicleType {
    ARRV,
    IFV,
    TANK,
    HELICOPTER,
    FIGHTER,
}

type PosPrecision = f32;

#[derive(Debug)]
pub struct Vehicle {
    pub id: ID,
    pub start_tick: usize,
    pub positions: Vec<Vec2<PosPrecision>>,
    max_speed: f32,
    order: Option<raw::Order>,
    order_executed: bool,
    terrain: TerrainHolder,
}

impl Vehicle {
    fn new(tick: usize, data: raw::Vehicle, terrain: &TerrainHolder, decoration: Option<raw::DecoratedVehicle>) -> Self {
        let mut vehicle = Self {
            id: data.id,
            start_tick: tick,
            positions: Vec::new(),
            order: None,
            order_executed: false,
            max_speed: data.maxSpeed.unwrap(),
            terrain: terrain.clone(),
        };
        vehicle.add_tick(tick, Some(data), decoration);
        vehicle
    }
    fn add_tick(&mut self, tick: usize, data: Option<raw::Vehicle>, decoration: Option<raw::DecoratedVehicle>) {
        if let Some(decoration) = decoration {
            if let Some(order) = decoration.order {
                self.order = Some(order);
            }
            if let Some(order_executed) = decoration.orderExecuted {
                self.order_executed = order_executed;
            }
        }
        if let Some(data) = data {
            let pos = if let (Some(x), Some(y)) = (data.x, data.y) {
                vec2(x as PosPrecision, y as PosPrecision)
            } else {
                self.execute_order(tick)
            };
            self.positions.push(pos);
        } else {
            if self.start_tick + self.positions.len() == tick {
                let mut pos = self.positions.last().unwrap().clone();
                if self.order_executed {
                    pos = self.execute_order(tick);
                }
                self.positions.push(pos);
            }
        }
    }
    fn execute_order(&self, tick: usize) -> Vec2<PosPrecision> {
        let order = self.order.as_ref().unwrap();
        if let raw::OrderType::MOVE = order.action {
            let terrain_k: f32 = 1.0;
            let mut speed = self.max_speed * terrain_k;
            if order.maxSpeed > 0.0 {
                speed = speed.min(order.maxSpeed);
            }
            let speed = vec2(order.x, order.y).normalize() * speed;
            let pos = *self.positions.last().unwrap() + speed;
            vec2(pos.x as PosPrecision, pos.y as PosPrecision)
        } else {
            // TODO
            vec2(0.0 as PosPrecision, 0.0 as PosPrecision)
        }
    }
}

#[derive(Debug)]
pub struct FixedVehicle {
    pub id: ID,
    pub pos: Vec2<PosPrecision>,
}

#[derive(Debug)]
pub struct Vehicles {
    terrain: TerrainHolder,
    map: HashMap<ID, Vehicle>,
}

impl Vehicles {
    pub fn new(terrain: &TerrainHolder) -> Self {
        Self {
            map: HashMap::new(),
            terrain: terrain.clone(),
        }
    }
    pub fn add_tick(&mut self,
                    tick: usize,
                    data: Option<Vec<raw::Vehicle>>,
                    decorations: Option<HashMap<String, raw::DecoratedVehicle>>,
                    effects: &Option<Vec<raw::Effect>>) {
        let mut decorations: HashMap<ID, raw::DecoratedVehicle> = {
            let mut result = HashMap::new();
            if let Some(decorations) = decorations {
                for decoration in decorations {
                    let id: ID = std::str::FromStr::from_str(&decoration.0).unwrap();
                    result.insert(id, decoration.1);
                }
            }
            result
        };

        let mut updated_ids: HashSet<ID> = HashSet::new();
        if let Some(data) = data {
            for v in data {
                let id = v.id;
                updated_ids.insert(id);
                if self.map.contains_key(&id) {
                    self.map.get_mut(&id).unwrap().add_tick(tick, Some(v), decorations.remove(&id));
                } else {
                    self.map.insert(id, Vehicle::new(tick, v, &self.terrain, decorations.remove(&id)));
                }
            }
        }
        if let &Some(ref effects) = effects {
            for effect in effects {
                if let raw::EffectType::VEHICLE_DEATH = effect.typ {
                    updated_ids.insert(effect.attributes.get("id").unwrap().as_i64().unwrap() as ID);
                }
            }
        }
        for vehicle in self.map.values_mut() {
            let id = vehicle.id;
            if !updated_ids.contains(&id) {
                vehicle.add_tick(tick, None, decorations.remove(&id));
            }
        }
    }
    pub fn get(&self, tick: usize) -> Vec<FixedVehicle> {
        let mut vehicles = Vec::new();
        for vehicle in self.map.values() {
            if vehicle.start_tick <= tick && tick < vehicle.start_tick + vehicle.positions.len() {
                vehicles.push(FixedVehicle {
                    id: vehicle.id,
                    pos: vehicle.positions[tick - vehicle.start_tick],
                });
            }
        }
        vehicles
    }
}