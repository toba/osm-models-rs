pub mod access;
pub mod relation;
pub mod restriction;
pub mod surface;
pub mod way_type;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Tag {
    key: String,
    value: String,
}

pub trait Tagged {
    /// Value of the tag or `None` if the tag isn't applied or it's applied
    /// without a value
    fn get_tag(&self, key: &str) -> Option<&str>;
    /// Whether tag is present even if the tag value is `None`
    fn has_tag(&self, key: &str) -> bool;
}

pub static JUNCTION_TYPE: &'static str = "junction";
pub static RAIL_TYPE: &'static str = "railway";
pub static TYPE: &'static str = "type";
pub static NAME: &'static str = "name";

pub mod travel_by {
    pub static BICYCLE: &'static str = "bicycle";
    pub static BUS: &'static str = "bus";
    pub static CAR: &'static str = "car";
    pub static FOOT: &'static str = "foot";
    pub static HORSE: &'static str = "horse";
    pub static MOTOR_CAR: &'static str = "motorcar";
    pub static MOTORCYCLE: &'static str = "motorcycle";
    pub static MOTOR_VEHICLE: &'static str = "motor_vehicle";
    pub static SERVICE_VEHICLE: &'static str = "psv";
    pub static TRAM: &'static str = "tram";
    pub static TRAIN: &'static str = "train";
    pub static VEHICLE: &'static str = "vehicle";
}

/// Elevation change of way (especially stairs)
pub mod incline {
    pub static KEY: &'static str = "incline";
    pub static UP: &'static str = "up";
    pub static DOWN: &'static str = "down";
}
