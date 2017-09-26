use ::*;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FacilityType {
    VEHICLE_FACTORY,
    CONTROL_CENTER,
}

#[derive(Debug)]
pub struct Facilities {
    pub factories: Vec<Vec2<f32>>,
    pub control_centers: Vec<Vec2<f32>>,
}

impl Facilities {
    pub const RADIUS: f32 = 32.0;

    pub fn new(facilities: Vec<raw::Facility>) -> Self {
        let mut factories = Vec::new();
        let mut control_centers = Vec::new();
        for facility in facilities {
            match facility.typ.unwrap() {
                FacilityType::VEHICLE_FACTORY => &mut factories,
                FacilityType::CONTROL_CENTER => &mut control_centers,
            }.push(vec2(facility.left.unwrap() + Self::RADIUS, facility.top.unwrap() + Self::RADIUS));
        }
        Self {
            factories,
            control_centers,
        }
    }
}