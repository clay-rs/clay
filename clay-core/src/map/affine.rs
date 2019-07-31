use vecmat::{map::*};
use crate::{pack::*, Map};


pub type Affine = Affine3<f64>;

impl Map for Affine {

}

impl Pack for Affine {
    fn size_int() -> usize {
        0
    }
    fn size_float() -> usize {
        3 + 3*9
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        let inv = self.linear.inverse();
        let t_inv = self.linear.transpose().inverse();
        Packer::new(buffer_int, buffer_float)
        .pack(&self.shift)
        .pack(&self.linear)
        .pack(&inv)
        .pack(&t_inv);
    }
}
