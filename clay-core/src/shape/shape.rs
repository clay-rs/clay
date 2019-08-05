use crate::{Pack, map::*, material::Material, object::Covered};


/// Shape of an object.
/// It defines the search of the point where ray intersects this shape.
pub trait Shape: Pack + Sized + 'static {
    /// Associated OpenCL code that contains necessary function definition.
    fn ocl_shape_code() -> String;
    /// Name of the function from the code that is used to find an intersection.
    fn ocl_shape_fn() -> String;
    /// Creates a new shape by applying some kind of mapping to previous one.
    ///
    /// Most common use case is applying affine transform to some unit shape.
    /// (*see `map::Affine`*)
    fn map<M: Map>(self, map: M) -> Mapper<Self, M> {
        Mapper::new(self, map)
    }
    /// Transforms the shape in an object by covering it with material.
    fn cover<M: Material>(self, material: M) -> Covered<Self, M> {
        Covered::new(self, material)
    }
}
