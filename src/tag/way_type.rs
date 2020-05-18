pub static KEY: &'static str = "highway";

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
///
/// This may also be a key to a more specific value about the foot path.
pub static FOOT_PATH: &'static str = "footway";

/// Type of `FOOT_PATH`
pub static SIDEWALK: &'static str = "sidewalk";

/// `FOOT_PATH` alternative
pub static PEDESTRIAN: &'static str = "pedestrian";

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
