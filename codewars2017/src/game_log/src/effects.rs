use ::*;

#[derive(Debug)]
pub struct AttackOrRepairEffect {
    pub start_tick: usize,
    pub vehicle_id: ID,
    pub target_id: ID,
}

#[derive(Debug)]
pub struct Effects {
    attacks: Vec<Option<Vec<AttackOrRepairEffect>>>,
    repairs: Vec<Option<Vec<AttackOrRepairEffect>>>,
}

fn add_to<T>(vec: &mut Vec<Option<Vec<T>>>, tick: usize, value: T) {
    while vec.len() <= tick {
        vec.push(None);
    }
    let cur = &mut vec[tick];
    if cur.is_none() {
        *cur = Some(Vec::new());
    }
    cur.as_mut().unwrap().push(value);
}

impl Effects {
    pub fn new() -> Self {
        Self {
            attacks: Vec::new(),
            repairs: Vec::new(),
        }
    }

    pub fn get_attacks(&self, tick: usize) -> Option<&Vec<AttackOrRepairEffect>> {
        if let Some(result) = self.attacks.get(tick) {
            result.as_ref()
        } else {
            None
        }
    }

    pub fn get_repairs(&self, tick: usize) -> Option<&Vec<AttackOrRepairEffect>> {
        if let Some(result) = self.repairs.get(tick) {
            result.as_ref()
        } else {
            None
        }
    }

    pub fn add_tick(&mut self, tick: usize, effects: Option<Vec<raw::Effect>>) {
        if let Some(effects) = effects {
            for effect in effects {
                match effect.typ {
                    raw::EffectType::VEHICLE_ATTACK => {
                        for tick_ext in tick..tick + 5 {
                            add_to(&mut self.attacks, tick_ext, AttackOrRepairEffect {
                                start_tick: tick,
                                vehicle_id: effect.attributes.get("vehicleId").unwrap().as_i64().unwrap() as ID,
                                target_id: effect.attributes.get("targetId").unwrap().as_i64().unwrap() as ID,
                            });
                        }
                    }
                    raw::EffectType::VEHICLE_REPAIR => {
                        add_to(&mut self.repairs, tick, AttackOrRepairEffect {
                            start_tick: tick,
                            vehicle_id: effect.attributes.get("vehicleId").unwrap().as_i64().unwrap() as ID,
                            target_id: effect.attributes.get("targetId").unwrap().as_i64().unwrap() as ID,
                        });
                    }
                    _ => {} // TODO
                }
            }
        }
    }
}