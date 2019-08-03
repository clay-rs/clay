use crate::{pack::*, Shape};


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

    /// OpenCL code associated with the cube.
    pub fn ocl_code() -> String {
        "#include <clay_core/shape/cube.h>".to_string()
    }
}

impl Pack for Cube {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}

impl Shape for Cube {
    fn ocl_shape_code() -> String {
        Self::ocl_code()
    }
    fn ocl_shape_fn() -> String {
        "cube_hit".to_string()
    }
}
