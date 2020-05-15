#![allow(dead_code)]

mod node;
mod overpass;
mod relation;
mod tag;
mod way;

mod item_type {
    pub static NODE: &'static str = "node";
    pub static WAY: &'static str = "way";
    pub static RELATION: &'static str = "relation";
}

use hashbrown::HashMap;

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
    nodes: HashMap<u16, node::Node>,
    /// Ways keyed to their ID
    ways: HashMap<u16, way::Way>,

    relations: Vec<relation::Relation>,
}
