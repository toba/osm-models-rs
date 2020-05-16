use crate::{node::Node, tag::Tagged, ElementID, TagMap, Timestamp};

pub struct Member<'a> {
    pub nodes: Vec<Node<'a>>,
    /// https://wiki.openstreetmap.org/wiki/Relation#Roles
    pub role: String,
}

/// Restrictions and boundaries defined among a collection of nodes
///
/// https://wiki.openstreetmap.org/wiki/Relation
/// https://wiki.openstreetmap.org/wiki/Relation:restriction
///
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

/// Relation member roles
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
