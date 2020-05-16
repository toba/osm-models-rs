use crate::{node::Node, tag::Tagged, ElementID, TagMap, Timestamp};

/// Relation member entry may refer to a single `Node` or a `Way` of nodes
pub struct Member<'a> {
    pub nodes: Vec<&'a Node<'a>>,
    /// https://wiki.openstreetmap.org/wiki/Relation#Roles
    pub role: &'a str,
}

/// Restrictions and boundaries defined among a collection of nodes
///
/// https://wiki.openstreetmap.org/wiki/Relation
/// https://wiki.openstreetmap.org/wiki/Relation:restriction
///
#[derive(Default)]
pub struct Relation<'a> {
    pub members: Vec<Member<'a>>,
    pub id: ElementID,
    pub timestamp: Timestamp,
    pub tags: TagMap<'a>,
}

impl<'a> Tagged for Relation<'a> {
    fn get_tag(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map_or(None, |t| t.as_deref())
    }

    fn has_tag(&self, key: &str) -> bool {
        self.tags.contains_key(key)
    }
}

/// A **role** is an optional textual field describing the function of a member
/// of the relation. For example, in North America, `east` indicates that a way
/// would be posted as East on the directional plate of a route numbering
/// shield. Or, multipolygon relation, `inner` and `outer` are used to specify
/// whether a way forms the inner or outer part of that polygon.
///
/// ### Multipolygon
/// [Multipolygons](https://wiki.openstreetmap.org/wiki/Relation:multipolygon)
/// are one of two methods to represent area areas in OpenStreetMap. While most
/// areas are represented as a single closed way closed way, almost all area features can also be mapped using multipolygon relations. This is needed when the area needs to exclude inner rings (holes), has multiple outer areas (exclaves), or uses more than ~2000 nodes.
///
/// In the multipolygon relation, the inner and outer roles are used to specify whether a member way forms the inner or outer part of that polygon enclosing an area. For example, an inner way could define an island in a lake (which is mapped as relation).
///
/// ### Bus Route
///
pub mod role {
    pub static FROM: &'static str = "from";
    pub static VIA: &'static str = "via";
    pub static TO: &'static str = "to";

    /// Relative polygon position
    pub static INNER: &'static str = "inner";
    /// Relative polygon position
    pub static OUTER: &'static str = "outer";
    /// Relative polygon position
    pub static SUBAREA: &'static str = "subarea";

    /// Direction of travel, e.g. for a bus route
    pub static FORWARD: &'static str = "forward";
    /// Direction of travel, e.g. for a bus route
    pub static BACKWARD: &'static str = "backward";

    pub static PLATFORM: &'static str = "platform";
}

/// https://wiki.openstreetmap.org/wiki/Relation:restriction
pub mod restriction {
    pub static NO_RIGHT_TURN: &'static str = "no_right_turn";
    pub static NO_LEFT_TURN: &'static str = "no_left_turn";
    pub static NO_U_TURN: &'static str = "no_u_turn";
    pub static NO_STRAIGHT: &'static str = "no_straight_on";
    pub static NO_ENTRY: &'static str = "no_entry";
    pub static NO_EXIT: &'static str = "no_exit";
    pub static ONLY_RIGHT_TURN: &'static str = "only_right_turn";
    pub static ONLY_LEFT_TURN: &'static str = "only_left_turn";
    pub static ONLY_STRAIGHT: &'static str = "only_straight_on";
}
