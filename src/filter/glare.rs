use std::collections::HashSet;
use ocl::{self, builders::KernelBuilder};
use crate::{Push, filter::Filter};


/// Glare blur.
///
/// Creates horizontal and vertical stripes around bright objects.
pub struct GlareFilter {
    strength: f64,
}

impl GlareFilter {
    pub fn new(strength: f64) -> Self {
        Self { strength }
    }
}

impl Filter for GlareFilter {
    fn inst_name() -> String {
        "glare_filter".to_string()
    }
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/filter/glare.h>".to_string()
    }
}

impl Push for GlareFilter {
    fn args_count() -> usize {
        1
    }
    fn args_def(kb: &mut KernelBuilder) {
        kb.arg(&0f32);
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        k.set_arg(i + 0, &(self.strength as f32))
        .map_err(|e| e.into())
    }
}
