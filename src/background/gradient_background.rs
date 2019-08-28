use std::collections::HashSet;
use nalgebra::{Vector3};
use ocl::{self, prm, builders::KernelBuilder};
use clay_core::{Push, Store, Context, Background};

#[derive(Debug, Clone)]
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

impl Store for GradientBackground {
    type Data = Self;
    fn create_data(&self, _context: &Context) -> clay_core::Result<Self::Data> {
        Ok(self.clone())
    }
    fn update_data(&self, _context: &Context, data: &mut Self::Data) -> clay_core::Result<()> {
        *data = self.clone();
        Ok(())
    }
}

impl Push for GradientBackground {
    fn args_def(kb: &mut KernelBuilder) {
        kb
        .arg(prm::Float3::zero()) // top color
        .arg(prm::Float3::zero()); // bottom color
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let (tc, bc) = (self.top.map(|d| d as f32), self.bottom.map(|d| d as f32));
        k.set_arg(i + 0, &prm::Float3::new(tc[0], tc[1], tc[2]))?;
        k.set_arg(i + 1, &prm::Float3::new(bc[0], bc[1], bc[2]))?;
        Ok(())
    }
    fn args_count() -> usize {
        2
    }
}
