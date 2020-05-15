use hashbrown::HashMap;

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
    id: u32,

    /// Latitude coordinate in degrees (North of equator is positive) using the
    /// standard WGS84 projection
    lat: f32,

    /// Longitude coordinate in degrees (East of Greenwich is positive) using
    /// the standard WGS84 projection. Note that the geographic poles will be
    /// exactly at latitude ±90 degrees but in that case the longitude will be
    /// set to an arbitrary value within this range.
    lon: f32,

    /// Altitude or elevation
    ele: Option<f32>,

    open: Option<bool>,
    date: Option<u32>,

    /// Time of the last modification
    /// example: "2016-12-31T23:59:59.999Z"
    timestamp: Option<u32>,

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
    tags: Option<HashMap<String, Option<String>>>,
}

impl Node {
    fn point(&self) -> (f32, f32) {
        (self.lat, self.lon)
    }
}
