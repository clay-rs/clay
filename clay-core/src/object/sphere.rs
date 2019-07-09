use vecmat::vec::*;

use super::Object;


/// Spherical geometry
#[derive(Clone, Debug)]
pub struct Sphere {
    /// Position of the center of the sphere
    pub pos: Vec3<f64>,
    /// Radius of the sphere
    pub rad: f64,
}

impl Object for Sphere {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 4 }

    fn ocl_trace_fn() -> String { "core_trace_sphere".to_string() }

    fn bounding_sphere(&self) -> Option<Sphere> {
        return Some(self.clone())
    }

    fn pack(&self, _buffer_int: &mut [u32], buffer_float: &mut [f32]) {
        for (dst, src) in buffer_float[0..3].iter_mut().zip(self.pos.data.iter()) {
            *dst = *src as f32;
        }
        buffer_float[3] = self.rad as f32;
    }
}
