use ::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum VehicleType {
    ARRV,
    IFV,
    TANK,
    HELICOPTER,
    FIGHTER,
}

type PosPrecision = u16;

#[derive(Debug)]
pub struct Vehicle {
    pub id: ID,
    pub start_tick: usize,
    pub positions: Vec<Vec2<PosPrecision>>,
}

impl Vehicle {
    fn new(tick: usize, data: raw::Vehicle) -> Self {
        let mut vehicle = Self {
            id: data.id,
            start_tick: tick,
            positions: Vec::new(),
        };
        vehicle.add_tick(tick, Some(data));
        vehicle
    }
    fn add_tick(&mut self, tick: usize, data: Option<raw::Vehicle>) {
        if let Some(data) = data {
            let x = if let Some(x) = data.x { x as PosPrecision } else { self.positions.last().unwrap().x };
            let y = if let Some(y) = data.y { y as PosPrecision } else { self.positions.last().unwrap().y };
            self.positions.push(vec2(x, y));
        } else {
            if self.start_tick + self.positions.len() == tick {
                let pos = self.positions.last().unwrap().clone();
                self.positions.push(pos);
            }
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
    pub map: HashMap<ID, Vehicle>
}

impl Vehicles {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn add_tick(&mut self, tick: usize, data: Option<Vec<raw::Vehicle>>) {
        if let Some(data) = data {
            for v in data {
                if self.map.contains_key(&v.id) {
                    self.map.get_mut(&v.id).unwrap().add_tick(tick, Some(v));
                } else {
                    self.map.insert(v.id, Vehicle::new(tick, v));
                }
            }
        } else {
            for vehicle in self.map.values_mut() {
                vehicle.add_tick(tick, None);
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