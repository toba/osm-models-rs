#![allow(dead_code)]

pub mod node;
pub mod overpass;
pub mod relation;
pub mod tag;
pub mod way;
pub mod item_type {
    pub static NODE: &'static str = "node";
    pub static WAY: &'static str = "way";
    pub static RELATION: &'static str = "relation";
}

use hashbrown::HashMap;

pub use node::Node;
pub use relation::{restriction, role, Relation};
pub use way::{way_type, Way};

/// Used for identifying the element. Element types have their own ID space,
/// so there could be a node with id=100 and a way with id=100, which are
/// unlikely to be related or geographically near to each other.
///
/// Positive (>0) values are used for all existing elements (and will remain
/// assigned when they are modified or deleted); negative values (<0) are
/// reserved (their scope limited to the current changeset and never stored
/// in the database) and only used when sending data to the OSM database for
/// identifying new objects to create and reference them in other created or
/// modified objects (the server will replace these temporary identifiers
/// sent by the editing application, by assigning an actual positive
/// identifier for each created object, and will return a mapping from the
/// negative identifiers used to their assigned positive identifiers).
///
/// https://wiki.openstreetmap.org/wiki/Elements#Common_attributes
///
pub type ElementID = i64;

/// All types of data element (nodes, ways and relations), as well as
/// changesets, can have tags. Tags describe the meaning of the particular
/// element to which they are attached.
///
/// A tag consists of two free format text fields; a `key` and a `value`.
/// Each of these are Unicode strings of up to 255 characters. For example,
/// `highway=residential` defines the way as a road whose main function is
/// to give access to people's homes. An element cannot have 2 tags with the
/// same `key`, the key's must be unique. For example, you cannot have an
/// element tagged both `amenity=restaurant` and `amenity=bar`.
///
/// There is no fixed dictionary of tags, but there are many conventions
/// documented on this wiki (starting with the Map Features page). Tag usage
/// can be measured with the Taginfo application. If there is more than one
/// way to tag a given feature, it's probably best to use the most common
/// approach.
///
/// Not all elements have tags. Nodes are often untagged if they are part of
/// ways. Both ways and nodes may be untagged if they are members of a
/// relation.
///
/// https://wiki.openstreetmap.org/wiki/Tags
///
pub type TagMap = HashMap<String, Option<String>>;

/// ISO 8601 time of the last modification (e.g. "2016-12-31T23:59:59.999Z")
/// https://wiki.openstreetmap.org/wiki/Elements#Common_attributes
pub type Timestamp = Option<String>;

/// Box-bounded OSM data download including
///
/// - All nodes that are inside a given bounding box and any relations that
///   reference them.
/// - All ways that reference at least one node that is inside a given bounding
///   box, any relations that reference them [the ways], and any nodes outside
///   the bounding box that the ways may reference.
/// - All relations that reference one of the nodes, ways or relations included
///   due to the above rules. (Does not apply recursively.)
///
/// https://wiki.openstreetmap.org/wiki/API_v0.6#Retrieving_map_data_by_bounding_box:_GET_.2Fapi.2F0.6.2Fmap
///
pub struct AreaData {
    /// Nodes keyed to their ID
    pub nodes: HashMap<u16, node::Node>,
    /// Ways keyed to their ID
    pub ways: HashMap<u16, way::Way>,

    pub relations: Vec<relation::Relation>,
}
