use std::collections::HashSet;
use nalgebra::{Vector3};
use crate::{prelude::*, map::*};


/// Shift along a specific vector.
pub struct Shift(pub Vector3<f64>);

impl From<Vector3<f64>> for Shift {
    fn from(x: Vector3<f64>) -> Self {
        Shift(x)
    }
}

impl Map for Shift {}

impl Instance<MapClass> for Shift {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/map/shift.h>".to_string()
    }
    fn inst_name() -> String {
        "shift".to_string()
    }
}

impl Pack for Shift {
    fn size_int() -> usize {
        Vector3::<f64>::size_int()
    }
    fn size_float() -> usize {
        Vector3::<f64>::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        self.0.pack_to(buffer_int, buffer_float);
    }
}
