use crate::{pack::*, Material};


#[derive(Clone, Debug, Default)]
pub struct Diffuse {}

impl Material for Diffuse {
    fn source() -> String {
        "#include <clay_core/material/diffuse.h>".to_string()
    }
    fn instance() -> String {
        "diffuse".to_string()
    }
}

impl Pack for Diffuse {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
