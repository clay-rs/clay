use crate::{Pack, map::*};


/// Shape of an object.
/// It defines the search of the point where ray intersects this shape.
pub trait Shape: Pack + Sized + 'static {
    /// Associated OpenCL code that contains necessary function definition.
    fn ocl_shape_code() -> String;
    /// Name of the function from the code that is used to find an intersection.
    fn ocl_shape_fn() -> String;
    // Create mapping of the shape
    fn map<M: Map + 'static>(self, map: M) -> Mapper<Self, M> {
        Mapper::new(self, map)
    }
}
