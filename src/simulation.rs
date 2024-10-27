/// Physics pipeline

use crate::copter::Quad;

pub struct Rapier {
}

impl Rapier {
    /// Place a physical object into simulation testbed
    /// TODO: consider using collections, like the ones used in Rapier (bodies, colliders, etc.)
    /// TODO: expand to a broader set of objects
    pub fn add_object(&mut self, quad: &Quad) {
    }

    pub fn time_us(&self) -> u64 {
        // TODO: return the actual time
        0u64
    }

    pub fn step(&mut self) {

    }
}
