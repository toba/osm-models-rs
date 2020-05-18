use crate::{node::Node, tag::Tagged, ElementID, TagMap, Timestamp};

/// Relation member entry may refer to a single `Node` or a `Way` of nodes
pub struct Member<'a> {
    pub nodes: Vec<&'a Node>,
    /// https://wiki.openstreetmap.org/wiki/Relation#Roles
    pub role: &'static str,
}

/// A **relation** is a group of [elements](https://wiki.openstreetmap.org/wiki/Elements).
/// To be more exact it is one of the core data elements that consists of one or
/// more [tags](https://wiki.openstreetmap.org/wiki/Tags) and also an **ordered
/// list** of one or more [nodes](https://wiki.openstreetmap.org/wiki/Node),
/// [ways](https://wiki.openstreetmap.org/wiki/Way) and/or relations as
/// **members** which is used to define logical or geographic relationships
/// between other elements. A member of a relation can optionally have a role
/// which describes the part that a particular feature plays within a relation.
///
/// https://wiki.openstreetmap.org/wiki/Relation
/// https://wiki.openstreetmap.org/wiki/Relation:restriction
///
#[derive(Default)]
pub struct Relation<'a> {
    pub members: Vec<Member<'a>>,
    pub id: ElementID,
    pub timestamp: Timestamp,
    pub tags: TagMap,
}

impl<'a> Tagged for Relation<'a> {
    fn get_tag(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map(|value| *value)
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
///
/// [Multipolygons](https://wiki.openstreetmap.org/wiki/Relation:multipolygon)
/// are one of two methods to represent [areas](https://wiki.openstreetmap.org/wiki/Area)
/// in OpenStreetMap. While most areas are represented as a single closed way
/// closed way, almost all area features can also be mapped using multipolygon
/// relations. This is needed when the area needs to exclude inner rings
/// (holes), has multiple outer areas (exclaves), or uses more than ~2000 nodes.
///
/// In the [multipolygon relation](https://wiki.openstreetmap.org/wiki/Relation:multipolygon),
/// the `inner` and `outer` roles are used to specify whether a member way forms
/// the inner or outer part of that polygon enclosing an area. For example, an
/// inner way could define an island in a lake (which is mapped as relation).
///
/// ### Bus Route
///
/// Each variation of a bus route itinerary is represented by a relation with
/// `type=route`, `route=bus` and `ref=*` and `operator=*` tags. The first
/// members in the route relation are the nodes representing the stops. These
/// are ordered in the way the vehicles travel along them. Then the ways are
/// added. In PT v2 the ways form an ordered sequence, along the stop nodes. The
/// ways don't get roles. If they form a continuous sequence this is apparent
/// from the continuous line along them (in JOSM's relation editor).
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

    pub static LABEL: &'static str = "label";

    pub static ADMIN_CENTER: &'static str = "admin_centre"; // correct sp
    pub static SUB_AREA: &'static str = "subarea";
}
