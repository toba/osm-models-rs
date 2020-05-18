//! **Access values** describe restrictions on the use of [highways](https://wiki.openstreetmap.org/wiki/Key:highway) and other
//! transportation routes ([railways](https://wiki.openstreetmap.org/wiki/Key:railway),
//! [waterways](https://wiki.openstreetmap.org/wiki/Key:waterway)), as well as
//! facilities such as buildings, building entrances, amenities and leisure
//! entities.
//!
//! Access values describe **legal** permissions/restrictions. What happens on
//! the ground may be different: for instance, many footpaths are used as *de
//! facto* bike paths, without a legal right to do so. (Various 'greyzone' tags
//! have been proposed to deal with such situations, but this is controversial
//! and is not described here.)
//!
//! Access restrictions can be very complex, for instance including type,
//! direction, size, and time of traffic - but with care most can be coded as
//! described here.
//!
//! https://wiki.openstreetmap.org/wiki/Key:access

pub static KEY: &'static str = "access";

/// Key that may be assigned "yes" or "no"
pub static ONE_WAY: &'static str = "oneway";

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
