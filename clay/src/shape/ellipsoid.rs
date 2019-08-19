use std::collections::HashSet;
use nalgebra::{Vector3, Matrix3, linalg::SVD};
use clay_core::{
    pack::*,
    class::*,
    map::*,
    shape::*,
};
use crate::{
    map::{Linear, Shift, Affine},
    shape::{UnitSphere, Sphere},
};

type EllipsoidBase = ShapeMapper<UnitSphere, Affine>;
pub struct Ellipsoid(EllipsoidBase);

impl Ellipsoid {
    pub fn new(ori: Matrix3<f64>, pos: Vector3<f64>) -> Self {
        Self::from(UnitSphere::new().map(Linear::from(ori).chain(Shift::from(pos))))
    }
}
impl From<EllipsoidBase> for Ellipsoid {
    fn from(base: EllipsoidBase) -> Self {
        Self(base)
    }
}

impl Shape for Ellipsoid {}

impl Instance<ShapeClass> for Ellipsoid {
    fn source(cache: &mut HashSet<u64>) -> String { EllipsoidBase::source(cache) }
    fn inst_name() -> String { EllipsoidBase::inst_name() }
}

impl Pack for Ellipsoid {
    fn size_int() -> usize { EllipsoidBase::size_int() }
    fn size_float() -> usize { EllipsoidBase::size_float() }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        self.0.pack_to(buffer_int, buffer_float);
    }
}

impl Bounded<Sphere> for Ellipsoid {
    fn bound(&self) -> Option<Sphere> {
        let rad = SVD::new(
            self.0.map.first.0,
            false, false,
        )
        .singular_values.as_slice().iter()
        .fold(std::f64::NAN, |a, b| f64::max(a, *b));
        Some(Sphere::new(rad, self.0.map.second.0))
    }
}
