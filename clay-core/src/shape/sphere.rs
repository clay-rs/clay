use std::collections::HashSet;
use nalgebra::{Matrix3, linalg::SVD};
use vecmat::{vec::*, mat::*};
use crate::{
    pack::*,
    class::*,
    map::*,
    shape::*,
};


#[derive(Clone, Debug, Default)]
/// Unit sphere - of radius one and centered at the origin.
///
/// This shape could be transformed to an arbitrary ellipsoid
/// by combining with the affine transform (*see `Shape::map()`*).
pub struct UnitSphere {}

impl UnitSphere {
    /// Creates new unit sphere
    pub fn new() -> Self {
        Self {}
    }
    fn source() -> String {
        "#include <clay_core/shape/sphere.h>".to_string()
    }
}

impl Shape for UnitSphere {}

impl Instance<ShapeClass> for UnitSphere {
    fn source(_: &mut HashSet<u64>) -> String { Self::source() }
    fn inst_name() -> String { "unitsphere".to_string() }
}

impl Pack for UnitSphere {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}


pub type Sphere = ShapeMapper<UnitSphere, Chain<Scale, Shift>>;

impl Sphere {
    pub fn build(rad: f64, pos: Vec3<f64>) -> Self {
        UnitSphere::new().map(Scale::from(rad).chain(Shift::from(pos)))
    }
}

impl Bound for Sphere {}
impl Instance<BoundClass> for Sphere {
    fn source(_: &mut HashSet<u64>) -> String { UnitSphere::source() }
    fn inst_name() -> String { "sphere".to_string() }
}

impl Target for Sphere {}
impl Instance<TargetClass> for Sphere {
    fn source(_: &mut HashSet<u64>) -> String { UnitSphere::source() }
    fn inst_name() -> String { "sphere_target".to_string() }
}


pub type Ellipsoid = ShapeMapper<UnitSphere, Affine>;

impl Ellipsoid {
    pub fn build(ori: Mat3<f64>, pos: Vec3<f64>) -> Self {
        UnitSphere::new().map(Linear::from(ori).chain(Shift::from(pos)))
    }
}

impl Bounded<Sphere> for Ellipsoid {
    fn bound(&self) -> Option<Sphere> {
        let rad = SVD::new(
            Matrix3::from_row_slice(&self.map.first.0.data),
            false, false,
        )
        .singular_values.as_slice().iter()
        .fold(std::f64::NAN, |a, b| f64::max(a, *b));
        Some(Sphere::build(rad, self.map.second.0))
    }
}
