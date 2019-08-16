use std::collections::HashSet;
use crate::{
    Pack, Packer,
    TypeHash, class::*,
    shape::*, material::*,
    object::*,
};


#[derive(Clone, Debug, Default)]
/// Object obtained by covering shape with material
pub struct Covered<S: Shape, M: Material> {
    pub shape: S,
    pub material: M,
}

impl<S: Shape, M: Material> Covered<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Self { shape, material }
    }

    fn shape_source(cache: &mut HashSet<u64>) -> String {
        [
            S::source(cache),
            ShapeClass::methods().into_iter().map(|method| {
                format!(
                    "#define {}_{} {}_{}",
                    Self::inst_name(), method,
                    S::inst_name(), method,
                )
            }).collect::<Vec<_>>().join("\n"),
        ].join("\n")
    }

    fn material_source(cache: &mut HashSet<u64>) -> String {
        [
            M::source(cache),
            MaterialClass::methods().into_iter().map(|method| {
                let cpref = format!("{}_{}", MaterialClass::name(), method).to_uppercase();
                [
                    &format!("{}_RET {}_{}(", cpref, Self::inst_name(), method),
                    &format!("\t{}_ARGS_DEF", cpref),
                    ") {",
                    &format!(
                        "\treturn {}_{}({}_ARGS_B({}, {}));",
                        M::inst_name(), method, cpref, S::size_int(), S::size_float(),
                    ),
                    "}",
                ].join("\n")
            }).collect::<Vec<_>>().join("\n"),
        ].join("\n")
    }
}

impl<S: Shape, M: Material> Object for Covered<S, M> {}

impl<S: Shape, M: Material> Instance<ObjectClass> for Covered<S, M> {
    fn source(cache: &mut HashSet<u64>) -> String {
        if !cache.insert(Self::type_hash()) {
            return String::new()
        }
        [
            Self::shape_source(cache),
            Self::material_source(cache),
        ].join("\n")
    }
    fn inst_name() -> String {
        format!("__covered_{:x}", Self::type_hash())
    }
}

impl<S: Shape, M: Material> Pack for Covered<S, M> {
    fn size_int() -> usize {
        S::size_int() + M::size_int()
    }
    fn size_float() -> usize {
        S::size_float() + M::size_float()
    }

    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        Packer::new(buffer_int, buffer_float)
        .pack(&self.shape)
        .pack(&self.material);
    }
}

impl<B: Bound, S: Shape + Bounded<B>, M: Material> Bounded<B> for Covered<S, M> {
    fn bound(&self) -> Option<B> {
        self.shape.bound()
    }
}

impl<T: Bound + Target, S: Shape + Bounded<T>, M: Material> Targeted<T> for Covered<S, M> {
    fn target(&self) -> Option<(T, f64)> {
        self.shape.bound()
        .map(|t| (t, self.material.brightness()))
    }
}
