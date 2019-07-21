use crate::{Pack, Shape};

/// Bounding shape that contains the whole object inside.
pub trait Bound: Pack {
    /// Associated OpenCL code that contains necessary function definition.
    fn ocl_bound_code() -> String;
    /// Name of the function from the code that is used to check bounds.
    fn ocl_bound_fn() -> String;
}

/// The shape that could be bounded by specified shape.
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
