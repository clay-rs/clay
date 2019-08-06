use crate::{pack::*, Shape};//, Bound};


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

impl Pack for Sphere {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}

impl Shape for Sphere {
    fn source() -> String {
        "#include <clay_core/shape/sphere.h>".to_string()
    }
    fn instance() -> String {
        "sphere".to_string()
    }
}
/*
impl Bound for Sphere {
    fn ocl_bound_code() -> String {
        Self::ocl_code()
    }
    fn ocl_bound_fn() -> String {
        "sphere_bound".to_string()
    }
}
*/
