use std::collections::HashSet;
use crate::{pack::*, class::*, material::*};


#[derive(Clone, Debug, Default)]
pub struct Diffuse {}

impl Material for Diffuse {
    fn brightness(&self) -> f64 {
        0.0
    }
}

impl Instance<MaterialClass> for Diffuse {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay_core/material/diffuse.h>".to_string()
    }
    fn inst_name() -> String {
        "diffuse".to_string()
    }
}

impl Pack for Diffuse {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
