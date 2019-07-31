use vecmat::vec::*;
use crate::{Pack, Material};

#[derive(Debug, Default)]
pub struct Mirror {
	pub color: Vec3<f64>,
}

impl Material for Mirror {
    fn ocl_material_code() -> String {
    	"#include <material/mirror.h>".to_string()
    }
    fn ocl_material_fn() -> String {
    	"mirror_emit".to_string()
    }
}

impl Pack for Mirror {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 3 }

    fn pack(&self, _buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        for (dst, src) in buffer_float[0..3].iter_mut().zip(self.color.data.iter()) {
            *dst = *src as f32;
        }
    }

    fn unpack(_buffer_int: &[i32], buffer_float: &[f32]) -> Self {
        let mut mirror = Self::default();
        for (dst, src) in mirror.color.data.iter_mut().zip(buffer_float[0..3].iter()) {
            *dst = *src as f64;
        }
        mirror
    }
}
