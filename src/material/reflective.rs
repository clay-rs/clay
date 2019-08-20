use std::collections::HashSet;
use clay_core::{pack::*, class::*, material::*};


#[derive(Clone, Debug, Default)]
pub struct Reflective {}

impl Material for Reflective {
    fn brightness(&self) -> f64 {
        0.0
    }
}

impl Instance<MaterialClass> for Reflective {
    fn source(_: &mut HashSet<u64>) -> String {
    	"#include <clay/material/reflective.h>".to_string()
    }
    fn inst_name() -> String {
        "reflective".to_string()
    }
}

impl Pack for Reflective {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
