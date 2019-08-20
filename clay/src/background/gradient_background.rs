use std::collections::HashSet;
use nalgebra::{Vector3};
use ocl::{self, prm, builders::KernelBuilder};
use clay_core::{Push, Background};

pub struct GradientBackground {
    pub top: Vector3<f64>,
    pub bottom: Vector3<f64>,
}

impl GradientBackground {
    pub fn new(top: Vector3<f64>, bottom: Vector3<f64>) -> Self {
        Self { top, bottom }
    }
}

impl Background for GradientBackground {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/background/gradient_background.h>".to_string()
    }
}

impl Push for GradientBackground {
    fn args_def(kb: &mut KernelBuilder) {
        kb
        .arg(prm::Float3::zero()) // top color
        .arg(prm::Float3::zero()); // bottom color
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let (tc, bc) = (self.top.map(|d| d as f32), self.bottom.map(|d| d as f32));
        k.set_arg(i + 0, &prm::Float3::new(tc[0], tc[1], tc[2]))?;
        k.set_arg(i + 1, &prm::Float3::new(bc[0], bc[1], bc[2]))?;
        Ok(())
    }
    fn args_count() -> usize {
        2
    }
}
