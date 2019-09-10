use std::collections::HashSet;
use nalgebra::{Vector3, Matrix3};
use crate::{
    prelude::*,
    map::*,
    shape::*,
};


type ParallelepipedBase = ShapeMapper<UnitCube, Affine>;
pub struct Parallelepiped(pub ParallelepipedBase);

impl Parallelepiped {
    pub fn new(ori: Matrix3<f64>, pos: Vector3<f64>) -> Self {
        Self::from(UnitCube::new().map(Linear::from(ori).chain(Shift::from(pos))))
    }
}
impl From<ParallelepipedBase> for Parallelepiped {
    fn from(base: ParallelepipedBase) -> Self {
        Self(base)
    }
}

impl Shape for Parallelepiped {}
impl Instance<ShapeClass> for Parallelepiped {
    fn source(cache: &mut HashSet<u64>) -> String {
        ParallelepipedBase::source(cache)
    }
    fn inst_name() -> String {
        ParallelepipedBase::inst_name()
    }
}
impl Pack for Parallelepiped {
    fn size_int() -> usize { ParallelepipedBase::size_int() }
    fn size_float() -> usize { ParallelepipedBase::size_float() }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        self.0.pack_to(buffer_int, buffer_float);
    }
}

impl Bounded<Sphere> for Parallelepiped {
    fn bound(&self) -> Option<Sphere> {
        let pos = self.0.map.second.0;
        let ori = self.0.map.first.0;
        let basis = ori.transpose();
        let mut rad = 0.0;
        for i in 0..8 {
            let mut data = [0.0; 3];
            for j in 0..3 {
                data[j] = 1.0 - 2.0*(((i << j) & 1) as f64);
            }
            let len = (basis*Vector3::from_column_slice(&data)).norm();
            if len > rad {
                rad = len;
            }
        }
        Some(Sphere::new(rad, pos))
    }
}
