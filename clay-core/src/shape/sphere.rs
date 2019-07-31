use crate::{pack::*, Shape, Bound};


/// Spherical shape
#[derive(Clone, Debug, Default)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }

    /// OpenCL code associated with the sphere.
    pub fn ocl_code() -> String {
        "#include <shape/sphere.h>".to_string()
    }
}

impl Pack for Sphere {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}

impl Shape for Sphere {
    fn ocl_shape_code() -> String {
        Self::ocl_code()
    }
    fn ocl_shape_fn() -> String {
        "sphere_hit".to_string()
    }
}

impl Bound for Sphere {
    fn ocl_bound_code() -> String {
        Self::ocl_code()
    }
    fn ocl_bound_fn() -> String {
        "sphere_bound".to_string()
    }
}
