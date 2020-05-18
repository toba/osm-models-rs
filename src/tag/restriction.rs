//! If a relation is tagged `type=restriction` then it will also have a
//! `restriction` tag with one of these values.
//!
//! If the first word is "no_", then no routing is possible from the "from" to
//! the "to" member. If it is "only_", then you know that the only routing
//! originating from the "from" member leads to the "to" member.
//!
//! https://wiki.openstreetmap.org/wiki/Relation:restriction

/// Indicates access generally disallowed or disallowed for a specific mode
/// (`tags::travel_by`) of transportation.
/// https://wiki.openstreetmap.org/wiki/Relation:restriction
pub static KEY: &'static str = "restriction";

/// Tag key for restriction exceptions. The value should be a mode of
/// transportation.
pub static EXCEPTION: &'static str = "except";

pub static NO_RIGHT_TURN: &'static str = "no_right_turn";
pub static NO_LEFT_TURN: &'static str = "no_left_turn";
pub static NO_U_TURN: &'static str = "no_u_turn";
pub static NO_STRAIGHT: &'static str = "no_straight_on";
pub static NO_ENTRY: &'static str = "no_entry";
pub static NO_EXIT: &'static str = "no_exit";
pub static ONLY_RIGHT_TURN: &'static str = "only_right_turn";
pub static ONLY_LEFT_TURN: &'static str = "only_left_turn";
pub static ONLY_STRAIGHT: &'static str = "only_straight_on";
