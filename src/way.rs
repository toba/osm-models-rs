use crate::{node::Node, tag::Tagged, ElementID, TagMap, Timestamp};

/// Collection of nodes representing a way of travel.
///
/// A way is an ordered list of nodes which normally also has at least one tag
/// or is included within a `relation`. A way can have between 2 and 2,000
/// nodes, although it's possible that faulty ways with zero or a single node
/// exist. A way can be open or closed. A closed way is one whose last node on
/// the way is also the first on that way. A closed way may be interpreted
/// either as a closed polyline, or an area, or both.
///
/// https://wiki.openstreetmap.org/wiki/Way
///
#[derive(Default)]
pub struct Way<'a> {
    pub nodes: Vec<&'a Node>,
    pub id: ElementID,
    pub name: Option<String>,
    pub timestamp: Timestamp,
    pub tags: Option<TagMap>,
}

impl<'a> Tagged for Way<'a> {
    fn get_tag(&self, key: &str) -> Option<&str> {
        self.tags
            .as_ref()
            .and_then(|tags| tags.get(key).map(|value| *value))
    }

    fn has_tag(&self, key: &str) -> bool {
        self.tags
            .as_ref()
            .map_or(false, |tags| tags.contains_key(key))
    }
}
