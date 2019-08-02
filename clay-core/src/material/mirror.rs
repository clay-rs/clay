use vecmat::vec::*;
use crate::{pack::*, Material};

#[derive(Clone, Debug, Default)]
pub struct Mirror {
	pub color: Vec3<f64>,
}

impl Material for Mirror {
    fn ocl_material_code() -> String {
    	"#include <clay_core/material/mirror.h>".to_string()
    }
    fn ocl_material_fn() -> String {
    	"mirror_emit".to_string()
    }
}

impl Pack for Mirror {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 3 }

    fn pack_to(&self, _buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        self.color.pack_float_to(buffer_float);
    }
}
