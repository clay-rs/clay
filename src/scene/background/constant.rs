use std::collections::HashSet;
use nalgebra::{Vector3};
use ocl::{self, prm, builders::KernelBuilder};
use crate::{prelude::*, Context, scene::Background};

#[derive(Debug, Clone)]
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
        "#include <clay/scene/background/constant_background.h>".to_string()
    }
}

impl Store for ConstantBackground {
    type Data = Self;
    fn create_data(&self, _context: &Context) -> clay_core::Result<Self::Data> {
        Ok(self.clone())
    }
    fn update_data(&self, _context: &Context, data: &mut Self::Data) -> clay_core::Result<()> {
        *data = self.clone();
        Ok(())
    }
}

impl Push for ConstantBackground {
    fn args_def(kb: &mut KernelBuilder) {
        kb.arg(prm::Float3::zero());
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let c = self.color.map(|d| d as f32);
        k.set_arg(i, &prm::Float3::new(c[0], c[1], c[2]))?;
        Ok(())
    }
    fn args_count() -> usize {
        1
    }
}
