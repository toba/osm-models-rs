use crate::{ElementID, TagMap, Timestamp};

/// A node is one of the core elements in the OpenStreetMap data model. It
/// consists of a single point in space defined by its latitude, longitude and
/// node id.
///
/// A third, optional dimension (altitude) can also be included: `key:ele`
/// (abrev. for "elevation"). A node can also be defined as part of a particular
/// `layer=*` or `level=*`, where distinct features pass over or under one
/// another; say, at a bridge.
///
/// https://wiki.openstreetmap.org/wiki/Node
///
pub struct Node {
    pub id: ElementID,

    /// Latitude coordinate in degrees (North of equator is positive) using the
    /// standard WGS84 projection
    pub lat: f32,

    /// Longitude coordinate in degrees (East of Greenwich is positive) using
    /// the standard WGS84 projection. Note that the geographic poles will be
    /// exactly at latitude ±90 degrees but in that case the longitude will be
    /// set to an arbitrary value within this range.
    pub lon: f32,

    /// Altitude or elevation
    pub ele: Option<f32>,

    pub open: Option<bool>,
    pub date: Option<u32>,
    pub timestamp: Timestamp,
    pub tags: Option<TagMap>,
}

impl Node {
    pub fn point(&self) -> (f32, f32) {
        (self.lat, self.lon)
    }
}
