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

/// https://wiki.openstreetmap.org/wiki/Key:access
pub static ACCESS: &'static str = "access";
/// `WayType` value
pub static ROAD_TYPE: &'static str = "highway";
pub static RAIL_TYPE: &'static str = "railway";
pub static JUNCTION_TYPE: &'static str = "junction";
pub static ONE_WAY: &'static str = "oneway";
/// Restriction exceptions
pub static EXCEPTION: &'static str = "except";
pub static TYPE: &'static str = "type";
/// Indicates access generally disallowed or disallowed for a specific mode
/// of transportation.
/// https://wiki.openstreetmap.org/wiki/Relation:restriction
pub static RESTRICTION: &'static str = "restriction";
pub static BICYCLE: &'static str = "bicycle";
pub static BUS: &'static str = "bus";
pub static FOOT: &'static str = "foot";
pub static HORSE: &'static str = "horse";
pub static MOTOR_CAR: &'static str = "motorcar";
pub static MOTORCYCLE: &'static str = "motorcycle";
pub static MOTOR_VEHICLE: &'static str = "motor_vehicle";
pub static SERVICE_VEHICLE: &'static str = "psv";
pub static VEHICLE: &'static str = "vehicle";

/// https://wiki.openstreetmap.org/wiki/Key:access
pub mod access {
    /// Access only for agricultural vehicles
    pub static AGRICULTURAL: &'static str = "agricultural";
    /// Public has an official, legally-enshrined right of access
    pub static ALLOWED: &'static str = "yes";
    /// Accewss only for customers
    pub static CUSTOMERS: &'static str = "customers";
    /// Access only for deliveries
    pub static DELIVERY: &'static str = "delivery";
    /// Access is legal but discouraged
    pub static DISCOURAGED: &'static str = "discouraged";
    /// Access only to specific destination
    pub static DESTINATION: &'static str = "destination";
    /// Only forestry traffic allowed
    pub static FORESTRY: &'static str = "forestry";
    /// No access to general public
    pub static NONE: &'static str = "no";
    /// Owner granted access
    pub static PERMISSIVE: &'static str = "permissive";
    /// Accessible only to individuals with permission
    pub static PRIVATE: &'static str = "private";
}
