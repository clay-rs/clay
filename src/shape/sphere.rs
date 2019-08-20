use std::collections::HashSet;
use nalgebra::{Vector3};
use clay_core::{
    pack::*,
    class::*,
    map::*,
    shape::*,
};
use crate::{
    map::{Scale, Shift},
    shape::UnitSphere,
};


type SphereBase = ShapeMapper<UnitSphere, Chain<Scale, Shift>>;
pub struct Sphere(pub SphereBase);

impl Sphere {
    pub fn new(rad: f64, pos: Vector3<f64>) -> Self {
        Self::from(UnitSphere::new().map(Scale::from(rad).chain(Shift::from(pos))))
    }
}
impl From<SphereBase> for Sphere {
    fn from(base: SphereBase) -> Self {
        Self(base)
    }
}

impl Shape for Sphere {}

impl Instance<ShapeClass> for Sphere {
    fn source(cache: &mut HashSet<u64>) -> String { SphereBase::source(cache) }
    fn inst_name() -> String { SphereBase::inst_name() }
}

impl Pack for Sphere {
    fn size_int() -> usize { SphereBase::size_int() }
    fn size_float() -> usize { SphereBase::size_float() }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        self.0.pack_to(buffer_int, buffer_float);
    }
}

impl Bound for Sphere {}
impl Instance<BoundClass> for Sphere {
    fn source(cache: &mut HashSet<u64>) -> String { UnitSphere::source(cache) }
    fn inst_name() -> String { "sphere".to_string() }
}

impl Target for Sphere {}
impl Instance<TargetClass> for Sphere {
    fn source(cache: &mut HashSet<u64>) -> String { UnitSphere::source(cache) }
    fn inst_name() -> String { "sphere_target".to_string() }
}
