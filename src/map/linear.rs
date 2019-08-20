use std::collections::HashSet;
use nalgebra::{Matrix3};
use clay_core::{pack::*, class::*, map::*};


pub struct Linear(pub Matrix3<f64>);

impl From<Matrix3<f64>> for Linear {
    fn from(x: Matrix3<f64>) -> Self {
        Linear(x)
    }
}

impl Map for Linear {}

impl Instance<MapClass> for Linear {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/map/linear.h>".to_string()
    }
    fn inst_name() -> String {
        "linear".to_string()
    }
}

impl Pack for Linear {
    fn size_int() -> usize {
        2*Matrix3::<f64>::size_int()
    }
    fn size_float() -> usize {
        2*Matrix3::<f64>::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        let inverse = self.0.try_inverse().unwrap();
        Packer::new(buffer_int, buffer_float)
        .pack(&self.0)
        .pack(&inverse);
    }
}
