//! Value of `type` tag on relation
//! https://wiki.openstreetmap.org/wiki/Types_of_relation

pub static EXCLUDE: &'static str = "should-be-excluded";

/// Bus routes, cycle routes and numbered highways
/// https://wiki.openstreetmap.org/wiki/Relation:route
pub static ROUTE: &'static str = "route";

/// Groups route variants in public transport
/// https://wiki.openstreetmap.org/wiki/Relation:route_master
pub static ROUTE_MASTER: &'static str = "route_master";

/// Groups boundaries and marks enclaves
/// https://wiki.openstreetmap.org/wiki/Relation:boundary
pub static BOUNDARY: &'static str = "boundary";

/// Relation to group elements of a [`waterway`](https://wiki.openstreetmap.org/wiki/Key:waterway)`=*`
/// https://wiki.openstreetmap.org/wiki/Relation:waterway
pub static WATERWAY: &'static str = "waterway";

/// Traffic enforcement devices (speed cameras, weight checks, etc.)
/// https://wiki.openstreetmap.org/wiki/Relation:enforcement
pub static ENFORCEMENT: &'static str = "enforcement";
