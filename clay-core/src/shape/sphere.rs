use std::collections::HashSet;
use crate::{pack::*, class::*, shape::*};


#[derive(Clone, Debug, Default)]
/// Unit sphere - of radius one and centered at the origin.
///
/// This shape could be transformed to an arbitrary ellipsoid
/// by combining with the affine transform (*see `Shape::map()`*).
pub struct Sphere {}

impl Sphere {
    /// Creates new unit sphere
    pub fn new() -> Self {
        Self {}
    }
}

impl Shape for Sphere {}

impl Instance<ShapeClass> for Sphere {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay_core/shape/sphere.h>".to_string()
    }
    fn inst_name() -> String {
        "sphere".to_string()
    }
}

impl Pack for Sphere {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
