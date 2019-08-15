use crate::{
    Pack,
    class::*,
};


/// Target is a shape that is able to uniformly draw a random vector
/// pointing to itself from any given point in space.
pub trait Target: Pack + Instance<TargetClass> {}

pub enum TargetClass {}
impl Class for TargetClass {
    fn name() -> String {
        "target".to_string()
    }
    fn methods() -> Vec<String> {
        vec![
            "size".to_string(),
            "sample".to_string(),
        ]
    }
}

/// The shape that could be put inside the specified bound.
pub trait Targeted<T: Target> {
    /// Returns target shape and the brightness.
    ///
    /// Brightness is used to compute ray attraction probability
    /// during the importance sampling process.
    fn target(&self) -> Option<(T, f64)>;
}
