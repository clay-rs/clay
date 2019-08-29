use std::collections::HashSet;
use nalgebra::{Vector3};
use ocl::{self, prm, builders::KernelBuilder};
use crate::{prelude::*, Context, scene::Background};

#[derive(Debug, Clone)]
pub struct GradientBackground {
    pub front: Vector3<f64>,
    pub back: Vector3<f64>,
    pub dir: Vector3<f64>,
}

impl GradientBackground {
    pub fn new(front: Vector3<f64>, back: Vector3<f64>, dir: Vector3<f64>) -> Self {
        Self { front, back, dir }
    }
}

impl Background for GradientBackground {
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/scene/background/gradient.h>".to_string()
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
        .arg(prm::Float3::zero()) // front color
        .arg(prm::Float3::zero()) // back color
        .arg(prm::Float3::zero()); // gradient direction
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let (fc, bc) = (self.front.map(|d| d as f32), self.back.map(|d| d as f32));
        let d = self.dir.map(|d| d as f32);
        k.set_arg(i + 0, &prm::Float3::new(fc[0], fc[1], fc[2]))?;
        k.set_arg(i + 1, &prm::Float3::new(bc[0], bc[1], bc[2]))?;
        k.set_arg(i + 2, &prm::Float3::new(d[0], d[1], d[2]))?;
        Ok(())
    }
    fn args_count() -> usize {
        3
    }
}
