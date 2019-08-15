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
}

impl<S: Shape, M: Material> Object for Covered<S, M> {}

impl<S: Shape, M: Material> Instance<ObjectClass> for Covered<S, M> {
    fn source(cache: &mut HashSet<u64>) -> String {
        if !cache.insert(Self::type_hash()) {
            return String::new()
        }
        let cpref = MaterialClass::name().to_uppercase();
        [
            S::source(cache),
            M::source(cache),
            [
                format!(
                    "#define {}_hit {}_hit",
                    Self::inst_name(),
                    S::inst_name(),
                ),
                "".to_string(),
                format!("{}_RET {}_emit(", cpref, Self::inst_name()),
                format!("\t{}_ARGS_DEF", cpref),
                ") {".to_string(),
                format!(
                    "\treturn {}_emit({}_ARGS_B({}, {}));",
                    M::inst_name(), cpref, S::size_int(), S::size_float(),
                ),
                "}".to_string(),
            ].join("\n")
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
