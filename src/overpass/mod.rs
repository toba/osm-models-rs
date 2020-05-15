/// Overpass API response
pub struct Response {
    version: f32,
    generator: String,
}

pub struct Node {
    typ: &'static str,
}

pub struct Way {
    nodes: Vec<u32>,
}
