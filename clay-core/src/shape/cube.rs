use std::collections::HashSet;
use vecmat::{vec::*, mat::*};
use crate::{
    pack::*,
    class::*,
    map::*,
    shape::*,
};


/// Unit cube - centered at the origin and of edge length two.
///
/// This shape could be transformed to an arbitrary parallelepiped
/// by combining with the affine transform  (*see `Shape::map()`*).
#[derive(Clone, Debug, Default)]
pub struct UnitCube {}

impl UnitCube {
    /// Creates new unit cube
    pub fn new() -> Self {
        Self {}
    }
}

impl Shape for UnitCube {}

impl Instance<ShapeClass> for UnitCube {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay_core/shape/cube.h>".to_string()
    }
    fn inst_name() -> String {
        "cube".to_string()
    }
}

impl Pack for UnitCube {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}


pub type Parallelepiped = ShapeMapper<UnitCube, Affine>;

impl Bounded<Sphere> for Parallelepiped {
    fn bound(&self) -> Option<Sphere> {
        let pos = self.map.second.0;
        let ori = self.map.first.0;
        let basis = (ori*Mat3::one()).transpose();
        let mut rad = 0.0;
        for i in 0..8 {
            let mut vertex = Vec3::default();
            for j in 0..3 {
                vertex.data[j] = 1.0 - 2.0*(((i << j) & 1) as f64);
            }
            let len = basis.dot(vertex).length();
            if len > rad {
                rad = len;
            }
        }
        Some(Sphere::build(pos, rad))
    }
}
