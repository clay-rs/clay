use crate::{
    Pack,
    class::*,
};


/// Attractor is a shape that is able to uniformly draw a random vector
/// pointing to itself from given point in space.
pub trait Attractor: Pack + Instance<AttractorClass> {}

pub enum AttractorClass {}
impl Class for AttractorClass {
    fn name() -> String {
        "attractor".to_string()
    }
    fn methods() -> Vec<String> {
        vec!["attract".to_string()]
    }
}
