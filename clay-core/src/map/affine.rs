pub use vecmat::{vec::*, mat::*};
pub use super::*;

pub type Affine = Chain<Linear, Shift>;

impl Affine {
    pub fn build(ori: Mat3<f64>, pos: Vec3<f64>) -> Self {
        Linear::from(ori).chain(Shift::from(pos))
    } 
}
