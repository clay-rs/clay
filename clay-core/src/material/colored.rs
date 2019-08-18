use std::collections::HashSet;
use nalgebra::Vector3;
use crate::{
    pack::*,
    class::*,
    material::*,
    TypeHash,
};


#[derive(Clone, Debug)]
pub struct Colored<M: Material> {
    pub material: M,
    pub color: Vector3<f64>,
}

impl<M: Material> Colored<M> {
    pub fn new(material: M, color: Vector3<f64>) -> Self {
        Self { material, color }
    }
}

impl<M: Material> Material for Colored<M> {
    fn brightness(&self) -> f64 {
        self.material.brightness()*
        self.color.data.iter().fold(0.0, |a, b| f64::max(a, *b))
    }
}

impl<M: Material> Instance<MaterialClass> for Colored<M> {
    fn source(cache: &mut HashSet<u64>) -> String {
        if !cache.insert(Self::type_hash()) {
            return String::new()
        }
        [
            M::source(cache),
            "#include <clay_core/material/colored.h>".to_string(),
            format!(
                "COLORED_MATERIAL_FN_DEF({}, {}, {}, {})",
                Self::inst_name(),
                M::inst_name(),
                M::size_int(),
                M::size_float(),
            ),
        ].join("\n")
    }
    fn inst_name() -> String {
        format!(
            "__{}_colored_{:x}",
            M::inst_name(),
            Self::type_hash(),
        )
    }
}

impl<M: Material> Pack for Colored<M> {
    fn size_int() -> usize { M::size_int() + 0 }
    fn size_float() -> usize { M::size_float() + 3 }

    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        self.material.pack_to(buffer_int, buffer_float);
        self.color.pack_float_to(&mut buffer_float[M::size_float()..]);
    }
}
