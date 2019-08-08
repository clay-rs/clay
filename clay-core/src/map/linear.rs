use std::collections::HashSet;
use vecmat::{mat::*};
use crate::{pack::*, class::*, map::*};


pub struct Linear(Mat3<f64>);

impl From<Mat3<f64>> for Linear {
    fn from(x: Mat3<f64>) -> Self {
        Linear(x)
    }
}

impl Map for Linear {}

impl Instance<MapClass> for Linear {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay_core/map/linear.h>".to_string()
    }
    fn inst_name() -> String {
        "linear".to_string()
    }
}

impl Pack for Linear {
    fn size_int() -> usize {
        2*Mat3::<f64>::size_int()
    }
    fn size_float() -> usize {
        2*Mat3::<f64>::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        let inverse = self.0.inverse();
        Packer::new(buffer_int, buffer_float)
        .pack(&self.0)
        .pack(&inverse);
    }
}
