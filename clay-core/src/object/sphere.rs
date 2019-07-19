use vecmat::vec::*;

use crate::{Pack, Object};


/// Spherical geometry
#[derive(Clone, Debug, Default)]
pub struct Sphere {
    /// Position of the center of the sphere
    pub pos: Vec3<f64>,
    /// Radius of the sphere
    pub rad: f64,
}

impl Pack for Sphere {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 4 }

    fn pack(&self, _buffer_int: &mut [u32], buffer_float: &mut [f32]) {
        for (dst, src) in buffer_float[0..3].iter_mut().zip(self.pos.data.iter()) {
            *dst = *src as f32;
        }
        buffer_float[3] = self.rad as f32;
    }

    fn unpack(_buffer_int: &[u32], buffer_float: &[f32]) -> Self {
        let mut sphere = Self::default();
        for (dst, src) in sphere.pos.data.iter().zip(buffer_float[0..3].iter()) {
            *dst = *src as f64;
        }
        sphere.rad = buffer_float[3] as f64;
        sphere
    }
}

impl Object for Sphere {
    fn bounds(&self) -> Option<Sphere> {
        return Some(self.clone())
    }
}
