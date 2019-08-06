use vecmat::{map::*};
use crate::{pack::*, class::*, map::*};


pub type Affine = Affine3<f64>;

impl Instance<MapClass> for Affine {
    fn source() -> String {
        "#include <clay_core/map/affine.h>".to_string()
    }
    fn inst_name() -> String {
        "affine".to_string()
    }
}

impl Map for Affine {}

impl Pack for Affine {
    fn size_int() -> usize {
        0
    }
    fn size_float() -> usize {
        3 + 9 + 9
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        let inverse = self.linear.inverse();
        Packer::new(buffer_int, buffer_float)
        .pack(&self.shift)
        .pack(&self.linear)
        .pack(&inverse);
    }
}
