use std::collections::HashSet;
use crate::{prelude::*, material::*};


#[derive(Clone, Debug, Default)]
pub struct Luminous {}

impl Material for Luminous {
    fn brightness(&self) -> f64 {
        1.0
    }
}

impl Instance<MaterialClass> for Luminous {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/material/luminous.h>".to_string()
    }
    fn inst_name() -> String {
        "luminous".to_string()
    }
}

impl Pack for Luminous {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
