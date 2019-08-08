use std::collections::HashSet;
use vecmat::{vec::*};
use crate::{Pack, class::*, map::*};


pub struct Shift(pub Vec3<f64>);

impl From<Vec3<f64>> for Shift {
    fn from(x: Vec3<f64>) -> Self {
        Shift(x)
    }
}

impl Map for Shift {}

impl Instance<MapClass> for Shift {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay_core/map/shift.h>".to_string()
    }
    fn inst_name() -> String {
        "shift".to_string()
    }
}

impl Pack for Shift {
    fn size_int() -> usize {
        Vec3::<f64>::size_int()
    }
    fn size_float() -> usize {
        Vec3::<f64>::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        self.0.pack_to(buffer_int, buffer_float);
    }
}
