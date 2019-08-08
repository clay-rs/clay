use std::collections::HashSet;
use crate::{TypeHash, pack::*, class::*, map::*};


pub struct Chain<F: Map, S: Map> {
    pub first: F,
    pub second: S,
}

impl<F: Map, S: Map> Chain<F, S> {
    pub fn new(first: F, second: S) -> Self {
        Self { first, second }
    }
}

impl<F: Map, S: Map> Map for Chain<F, S> {}

impl<F: Map, S: Map> Instance<MapClass> for Chain<F, S> {
    fn source(cache: &mut HashSet<u64>) -> String {
        if !cache.insert(Self::type_hash()) {
            return String::new()
        }
        [
            F::source(cache),
            S::source(cache),
            "#include <clay_core/map/chain.h>".to_string(),
            format!(
                "MAP_CHAIN({}, {}, {}, {}, {})",
                Self::inst_name(),
                F::inst_name(),
                S::inst_name(),
                F::size_int(),
                F::size_float(),
            ),
        ].join("\n")
    }
    fn inst_name() -> String {
        format!(
            "__{}_{}_{:x}",
            F::inst_name(),
            S::inst_name(),
            Self::type_hash(),
        )
    }
}

impl<F: Map, S: Map> Pack for Chain<F, S> {
    fn size_int() -> usize {
        F::size_int() + S::size_int()
    }
    fn size_float() -> usize {
        F::size_float() + S::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        Packer::new(buffer_int, buffer_float)
        .pack(&self.first)
        .pack(&self.second);
    }
}
