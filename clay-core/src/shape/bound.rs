use crate::{
    Pack, Shape,
    class::*,
};


/// Bounding shape that contains the whole object inside.
pub trait Bound: Pack + Instance<BoundClass> {}

pub enum BoundClass {}
impl Class for BoundClass {
    fn name() -> String {
        "bound".to_string()
    }
    fn methods() -> Vec<String> {
        vec!["bound".to_string()]
    }
}

/// The shape that could be put inside the specified bound.
pub trait Bounded: Shape {
    /// Bounding shape.
    type Bound: Bound;

    /// Returns bounding shape instance.
    ///
    /// If the shape is borderless and doesn't fit into any bounding shape
    /// then `None` should be returned.
    fn bound(&self) -> Option<Self::Bound>;
}

impl<T: Bound + Shape + Clone> Bounded for T {
    type Bound = T;
    fn bound(&self) -> Option<Self::Bound> {
        Some(self.clone())
    }
}
