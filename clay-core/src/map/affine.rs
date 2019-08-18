pub use nalgebra::{Vector3, Matrix3};
pub use super::*;

pub type Affine = Chain<Linear, Shift>;

impl Affine {
    pub fn build(ori: Matrix3<f64>, pos: Vector3<f64>) -> Self {
        Linear::from(ori).chain(Shift::from(pos))
    } 
}
