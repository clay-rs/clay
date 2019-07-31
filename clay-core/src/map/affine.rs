use vecmat::{map::*};
use crate::{pack::*, Map};


pub type Affine = Affine3<f64>;

impl Map for Affine {
    fn ocl_map_code() -> String {
        "#include <map/affine.h>".to_string()
    }
    fn ocl_map_pref() -> String {
        "affine".to_string()
    }
}

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
