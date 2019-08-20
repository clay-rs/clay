use std::collections::HashSet;
use nalgebra::{Vector3};
use ocl::{self, prm, builders::KernelBuilder};
use clay_core::{Push, Background};

pub struct ConstantBackground {
    pub color: Vector3<f64>,
}

impl ConstantBackground {
    pub fn new(color: Vector3<f64>) -> Self {
        Self { color }
    }
}

impl Background for ConstantBackground {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/background/constant_background.h>".to_string()
    }
}

impl Push for ConstantBackground {
    fn args_def(kb: &mut KernelBuilder) {
        kb.arg(prm::Float3::zero());
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let c = self.color.map(|d| d as f32);
        k.set_arg(i, &prm::Float3::new(c[0], c[1], c[2]))?;
        Ok(())
    }
    fn args_count() -> usize {
        1
    }
}
