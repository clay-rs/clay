use std::collections::HashSet;
use ocl::{self, builders::KernelBuilder};
use crate::{Push, filter::Filter};


/// Filter that doesn't change picture. Used as a placeholder.
#[derive(Default)]
pub struct IdentityFilter {}

impl IdentityFilter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Filter for IdentityFilter {
    fn inst_name() -> String {
        "identity_filter".to_string()
    }
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay_core/filter/identity.h>".to_string()
    }
}

impl Push for IdentityFilter {
    fn args_count() -> usize {
        1
    }
    fn args_def(kb: &mut KernelBuilder) {
        kb.arg(&0i32);
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        k.set_arg(i, &0i32).map_err(|e| e.into())
    }
}
