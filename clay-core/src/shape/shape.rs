use crate::{
    Pack, 
    map::*, 
    material::Material, 
    object::Covered,
    class::*,
};


/// Shape of an object.
/// It defines the search of the point where ray intersects this shape.
pub trait Shape: Pack + Instance<ShapeClass> + Sized + 'static {
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

pub enum ShapeClass {}
impl Class for ShapeClass {
    fn name() -> String {
        "shape".to_string()
    }
    fn methods() -> Vec<String> {
        vec!["hit".to_string()]
    }
}
