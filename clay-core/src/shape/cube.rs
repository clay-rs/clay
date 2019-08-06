use crate::{pack::*, class::*, shape::*};


/// Unit cube - centered at the origin and of edge length two.
///
/// This shape could be transformed to an arbitrary parallelepiped
/// by combining with the affine transform  (*see `Shape::map()`*).
#[derive(Clone, Debug, Default)]
pub struct Cube {}

impl Cube {
    /// Creates new unit cube
    pub fn new() -> Self {
        Self {}
    }
}

impl Shape for Cube {}

impl Instance<ShapeClass> for Cube {
    fn source() -> String {
        "#include <clay_core/shape/cube.h>".to_string()
    }
    fn inst_name() -> String {
        "cube".to_string()
    }
}

impl Pack for Cube {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
