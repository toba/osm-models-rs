use crate::node::Node;
use hashbrown::HashMap;

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
pub struct Way {
    nodes: Vec<Node>,

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

pub mod way_type {
    /// For horse riders. Equivalent to `highway=path` + `horse=designated`.
    pub static HORSE_PATH: &'static str = "bridleway";

    /// For designated cycleways. Add `foot=*` only if
    /// default-access-restrictions do not apply.
    pub static BICYCLE_PATH: &'static str = "cycleway";

    /// For designated footpaths; i.e., mainly/exclusively for pedestrians. This
    /// includes walking tracks and gravel paths. If bicycles are allowed as
    /// well, you can indicate this by adding a `bicycle=yes` tag. Should not be
    /// used for paths where the primary or intended usage is unknown. Use
    /// `highway=pedestrian` for pedestrianised roads in shopping or residential
    /// areas and `highway=track` if it is usable by agricultural or similar
    /// vehicles.
    pub static FOOT_PATH: &'static str = "footway";

    pub static LIGHT_RAIL: &'static str = "light_rail";

    /// A restricted access major divided highway, normally with 2 or more
    /// running lanes plus emergency hard shoulder. Equivalent to the Freeway,
    /// Autobahn, etc.
    pub static FREEWAY: &'static str = "motorway";

    pub static NARROW_GAUGE: &'static str = "narrow_gauge";
    pub static PATH: &'static str = "path";

    /// Links between larger towns
    pub static PRIMARY: &'static str = "primary";

    pub static RAIL: &'static str = "rail";

    /// Roads which serve as an access to housing, without function of
    /// connecting settlements. Often lined with housing.
    pub static RESIDENTIAL: &'static str = "residential";

    /// Links between towns
    pub static SECONDARY: &'static str = "secondary";

    /// For access roads to, or within an industrial estate, camp site, business
    /// park, car park etc. Can be used in conjunction with `service=*` to
    /// indicate the type of usage and with access=* to indicate who can use it
    /// and in what circumstances.
    pub static SERVICE_ROAD: &'static str = "service";

    /// For flights of steps (stairs) on footways. Use with `step_count=*` to
    /// indicate the number of steps.
    pub static STAIRS: &'static str = "steps";

    pub static SUBWAY: &'static str = "subway";
    pub static TERTIARY: &'static str = "tertiary";

    /// Roads for mostly agricultural or forestry uses. To describe the quality
    /// of a track, see `tracktype=*`. Note: Although tracks are often rough
    /// with unpaved surfaces, this tag is not describing the quality of a road
    /// but its use. Consequently, if you want to tag a general use road, use
    /// one of the general highway values instead of track.
    pub static TWO_TRACK: &'static str = "track";

    pub static TRAM: &'static str = "tram";

    /// The most important roads in a country's system that aren't motorways.
    /// (Need not necessarily be a divided highway.)
    pub static TRUNK: &'static str = "trunk";

    /// The least important through roads in a country's system — i.e. minor
    /// roads of a lower classification than tertiary, but which serve a purpose
    /// other than access to properties. (Often link villages and hamlets.)
    ///
    /// The word 'unclassified' is a historical artefact of the UK road system
    /// and does not mean that the classification is unknown; you can use
    /// `highway=road` for that.
    pub static MINOR: &'static str = "unclassified";
}

/// Modes of travel
pub mod travel_mode {
    use crate::tag;

    pub static CAR: &'static str = "car";
    pub static BUS: &'static str = tag::BUS;
    pub static BICYCLE: &'static str = tag::BICYCLE;
    pub static HORSE: &'static str = tag::HORSE;
    pub static WALK: &'static str = tag::FOOT;
    pub static MOTORCYCLE: &'static str = tag::MOTORCYCLE;
    pub static TRAM: &'static str = "tram";
    pub static TRAIN: &'static str = "train";
}
