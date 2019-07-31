use vecmat::vec::*;

use crate::{pack::*, Shape, Bound};


/// Spherical shape
#[derive(Clone, Debug, Default)]
pub struct Sphere {
    /// Position of the center of the sphere
    pub pos: Vec3<f64>,
    /// Radius of the sphere
    pub rad: f64,
}

impl Sphere {
    /// OpenCL code associated with the sphere.
    pub fn ocl_code() -> String {
        [
            format!("#define SPHERE_SIZE_INT {}", Self::size_int()),
            format!("#define SPHERE_SIZE_FLOAT {}", Self::size_float()),
            "#include <object/sphere.h>".to_string(),
        ].join("\n")
    }
}

impl Pack for Sphere {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 4 }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        Packer::new(buffer_int, buffer_float)
        .pack(&self.pos)
        .pack(&self.rad);
    }
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
