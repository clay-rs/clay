use std::collections::HashSet;
use crate::{
    prelude::*,
    shape::*,
};


/// Unit sphere - of radius one and centered at the origin.
///
/// This shape could be transformed to an arbitrary ellipsoid
/// by combining with the affine transform (*see `Shape::map()`*).
#[derive(Clone, Debug, Default)]
pub struct UnitSphere {}

impl UnitSphere {
    /// Creates new unit sphere
    pub fn new() -> Self {
        Self {}
    }
    fn source() -> String {
        "#include <clay/shape/sphere.h>".to_string()
    }
}

impl Shape for UnitSphere {}

impl Instance<ShapeClass> for UnitSphere {
    fn source(_: &mut HashSet<u64>) -> String { Self::source() }
    fn inst_name() -> String { "unit_sphere".to_string() }
}

impl Pack for UnitSphere {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
