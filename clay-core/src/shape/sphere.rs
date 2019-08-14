use std::collections::HashSet;
use vecmat::{vec::*};
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

    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay_core/shape/sphere.h>".to_string()
    }
    fn inst_name() -> String {
        "sphere".to_string()
    }
}

impl Shape for UnitSphere {}

impl Instance<ShapeClass> for UnitSphere {
    fn source(cache: &mut HashSet<u64>) -> String { Self::source(cache) }
    fn inst_name() -> String { Self::inst_name() }
}

impl Pack for UnitSphere {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}


pub type Sphere = ShapeMapper<UnitSphere, Chain<Scale, Shift>>;

impl Sphere {
    pub fn build(pos: Vec3<f64>, rad: f64) -> Self {
        UnitSphere::new().map(Scale::from(rad).chain(Shift::from(pos)))
    }
}

impl Bound for Sphere {}

impl Instance<BoundClass> for Sphere {
    fn source(cache: &mut HashSet<u64>) -> String { UnitSphere::source(cache) }
    fn inst_name() -> String { UnitSphere::inst_name() }
}

impl Target for Sphere {}

impl Instance<TargetClass> for Sphere {
    fn source(cache: &mut HashSet<u64>) -> String { UnitSphere::source(cache) }
    fn inst_name() -> String { UnitSphere::inst_name() }
}


pub type Ellipsoid = ShapeMapper<UnitSphere, Affine>;
