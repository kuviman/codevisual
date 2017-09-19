use ::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VehicleType {
    ARRV,
    IFV,
    TANK,
    HELICOPTER,
    FIGHTER,
}

#[derive(Debug)]
pub struct Vehicle {
    pub id: ID,
    pub start_tick: usize,
    pub positions: Vec<Vec2<u16>>,
    angles: Vec<u8>,
    max_speed: f32,
    last_pos: Vec2<f32>,
    last_angle: f32,
    order: Option<raw::Order>,
    order_executed: bool,
    terrain: TerrainHolder,
    weather: WeatherHolder,
    aerial: bool,
    radius: f32,
    typ: VehicleType,
    player_id: ID,
}

impl Vehicle {
    fn new(tick: usize,
           data: raw::Vehicle,
           terrain: &TerrainHolder,
           weather: &WeatherHolder,
           decoration: Option<raw::DecoratedVehicle>) -> Self {
        let mut vehicle = Self {
            id: data.id,
            start_tick: tick,
            positions: Vec::new(),
            angles: Vec::new(),
            order: None,
            order_executed: false,
            max_speed: data.maxSpeed.unwrap(),
            terrain: terrain.clone(),
            weather: weather.clone(),
            aerial: data.aerial.unwrap(),
            radius: data.radius.unwrap(),
            typ: data.typ.unwrap(),
            player_id: data.playerId.unwrap(),
            last_pos: vec2(data.x.unwrap(), data.y.unwrap()),
            last_angle: 0.0,
        };
        vehicle.add_tick(tick, Some(data), decoration);
        vehicle
    }
    fn add_pos(&mut self, pos: Vec2<f32>) {
        let mut angle = self.last_angle;
        let dv = pos - self.last_pos;
        if dv.len() > 0.1 {
            angle = f32::atan2(dv.y, dv.x);
            if angle < 0.0 {
                angle += 2.0 * std::f32::consts::PI;
            }
        }
        self.positions.push(vec2((pos.x * 10.0) as u16, (pos.y * 10.0) as u16));
        self.angles.push((angle * 255.0 / 2.0 / std::f32::consts::PI) as u8);
        self.last_pos = pos;
        self.last_angle = angle;
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
                vec2(x, y)
            } else {
                if self.order_executed {
                    self.execute_order(tick)
                } else {
                    self.last_pos
                }
            };
            self.add_pos(pos);
        } else {
            if self.start_tick + self.positions.len() == tick {
                let mut pos = self.last_pos;
                if self.order_executed {
                    pos = self.execute_order(tick);
                }
                self.add_pos(pos);
            }
        }
    }
    fn execute_order(&self, tick: usize) -> Vec2<f32> {
        let pos: Vec2<f32> = self.last_pos;
        let terrain_k: f32 = {
            const CELL_SIZE: f32 = 32.0;
            let cell = vec2(clamp((pos.x / CELL_SIZE) as usize, 0, self.terrain.len() - 1),
                            clamp((pos.y / CELL_SIZE) as usize, 0, self.terrain[0].len() - 1));
            if self.aerial {
                match self.weather[cell.x][cell.y] {
                    WeatherType::CLEAR => 1.0,
                    WeatherType::CLOUD => 0.8,
                    WeatherType::RAIN => 0.6,
                }
            } else {
                match self.terrain[cell.x][cell.y] {
                    TerrainType::PLAIN => 1.0,
                    TerrainType::FOREST => 0.8,
                    TerrainType::SWAMP => 0.6,
                }
            }
        };
        let order = self.order.as_ref().unwrap();
        let mut speed = self.max_speed * terrain_k;
        if let raw::OrderType::MOVE = order.action {
            if order.maxSpeed > 0.0 {
                speed = speed.min(order.maxSpeed);
            }
            let mut dv = vec2(order.targetX, order.targetY) - pos;
            if dv.len() > speed {
                dv = dv.normalize() * speed;
            }
            pos + dv
        } else {
            let origin = vec2(order.x, order.y);
            let r = pos - origin;
            let angular_speed = if order.maxAngularSpeed > 0.0 {
                order.maxAngularSpeed.min(speed / r.len())
            } else {
                if order.maxSpeed > 0.0 {
                    speed = speed.min(order.maxSpeed);
                }
                speed / r.len()
            };
            let nr = Vec2::rotated(r, angular_speed * order.angle.signum());
            let npos = origin + nr;
            if (vec2(order.targetX, order.targetY) - pos).len() < (npos - pos).len() {
                vec2(order.targetX, order.targetY)
            } else {
                npos
            }
        }
    }
}

#[derive(Debug)]
pub struct FixedVehicle {
    pub typ: VehicleType,
    pub id: ID,
    pub pos: Vec2<f32>,
    pub radius: f32,
    pub player_id: ID,
    pub aerial: bool,
    pub angle: f32,
}

type MMAP<K, V> = VecMap<K, V>;

#[derive(Debug)]
pub struct Vehicles {
    terrain: TerrainHolder,
    weather: WeatherHolder,
    map: MMAP<ID, Vehicle>,
    decorations: MMAP<ID, raw::DecoratedVehicle>,
    last_updated_tick: MMAP<ID, usize>,
}

impl Vehicles {
    pub fn new(terrain: &TerrainHolder, weather: &WeatherHolder) -> Self {
        Self {
            map: VecMap::new(),
            terrain: terrain.clone(),
            weather: weather.clone(),
            decorations: VecMap::new(),
            last_updated_tick: VecMap::new(),
        }
    }
    pub fn add_tick(&mut self,
                    tick: usize,
                    data: Option<Vec<raw::Vehicle>>,
                    decorations: Option<HashMap<String, raw::DecoratedVehicle>>,
                    effects: &Option<Vec<raw::Effect>>) {
        if let Some(decorations) = decorations {
            for decoration in decorations {
                let id: ID = std::str::FromStr::from_str(&decoration.0).unwrap();
                self.decorations.insert(id, decoration.1);
            }
        }

        if let Some(data) = data {
            for v in data {
                let id = v.id;
                self.last_updated_tick.insert(id, tick);
                if self.map.contains_key(&id) {
                    self.map.get_mut(&id).unwrap().add_tick(tick, Some(v), self.decorations.remove(&id));
                } else {
                    self.map.insert(id, Vehicle::new(tick, v, &self.terrain, &self.weather, self.decorations.remove(&id)));
                }
            }
        }
        if let &Some(ref effects) = effects {
            for effect in effects {
                if let raw::EffectType::VEHICLE_DEATH = effect.typ {
                    self.last_updated_tick.insert(effect.attributes.get("id").unwrap().as_i64().unwrap() as ID, tick);
                }
            }
        }
        for vehicle in self.map.values_mut() {
            let id = vehicle.id;
            if *self.last_updated_tick.get(&id).unwrap_or(&usize::max_value()) != tick {
                vehicle.add_tick(tick, None, self.decorations.remove(&id));
            }
        }
    }
    pub fn get(&self, tick: usize) -> Vec<FixedVehicle> {
        let mut vehicles = Vec::new();
        for vehicle in self.map.values() {
            if vehicle.start_tick <= tick && tick < vehicle.start_tick + vehicle.positions.len() {
                vehicles.push(FixedVehicle {
                    id: vehicle.id,
                    pos: {
                        let pos = vehicle.positions[tick - vehicle.start_tick];
                        vec2(pos.x as f32 / 10.0, pos.y as f32 / 10.0)
                    },
                    radius: vehicle.radius,
                    typ: vehicle.typ,
                    player_id: vehicle.player_id,
                    aerial: vehicle.aerial,
                    angle: vehicle.angles[tick - vehicle.start_tick] as f32 * (2.0 * std::f32::consts::PI / 255.0),
                });
            }
        }
        vehicles
    }
}