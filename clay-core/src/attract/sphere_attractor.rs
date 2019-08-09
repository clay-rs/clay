use std::collections::HashSet;
use crate::{pack::*, class::*, map::*, shape::*, attract::*};

type BSphere = Mapper<Sphere, Chain<Scale, Shift>>;

pub struct SphereAttractor {
    pub sphere: BSphere,
    pub target: usize,
}

impl SphereAttractor {
    pub fn new(sphere: BSphere, target: usize) -> Self {
        Self { sphere, target }
    }
}

impl Attractor for SphereAttractor {}

impl Instance<AttractorClass> for SphereAttractor {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay_core/attract/sphere_attractor.h>".to_string()
    }
    fn inst_name() -> String {
        "sphere".to_string()
    }
}

impl Pack for SphereAttractor {
    fn size_int() -> usize {
        1 + BSphere::size_int()
    }
    fn size_float() -> usize {
        BSphere::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        buffer_int.pack(&(self.target as u32));
        self.sphere.pack_to(&mut buffer_int[1..], buffer_float);
    }
}
