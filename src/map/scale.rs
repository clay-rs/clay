use std::collections::HashSet;
use crate::{prelude::*, map::*};


/// Isotropic scaling.
pub struct Scale(pub f64);

impl From<f64> for Scale {
    fn from(x: f64) -> Self {
        Scale(x)
    }
}

impl Map for Scale {}

impl Instance<MapClass> for Scale {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/map/scale.h>".to_string()
    }
    fn inst_name() -> String {
        "scale".to_string()
    }
}

impl Pack for Scale {
    fn size_int() -> usize {
        0
    }
    fn size_float() -> usize {
        1
    }
    fn pack_to(&self, _buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        buffer_float.pack(&self.0);
    }
}
