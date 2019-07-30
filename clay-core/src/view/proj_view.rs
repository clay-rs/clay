use ocl::{self, prm, builders::KernelBuilder};
use vecmat::{vec::*, mat::*};
use crate::{Push, View};

pub struct ProjView {
    pub pos: Vec3<f64>,
    pub ori: Mat3<f64>,
}

impl View for ProjView {
	fn ocl_emit_code() -> String {
		"#include <view.h>\n".to_string()
	}
}

impl Push for ProjView {
	fn args_def(kb: &mut KernelBuilder) {
        kb
        .arg(prm::Float3::zero())
        .arg(prm::Float16::zero());
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let mapf = self.ori.map(|x| x as f32);
        let mut map16 = [0f32; 16];
        map16[0..3].copy_from_slice(&mapf.row(0).data);
        map16[4..7].copy_from_slice(&mapf.row(1).data);
        map16[8..11].copy_from_slice(&mapf.row(2).data);

        k.set_arg(i + 0, &prm::Float3::from(self.pos.map(|e| e as f32).data))?;
        k.set_arg(i + 1, &prm::Float16::from(map16))?;

        Ok(())
    }
    fn args_count() -> usize {
        2
    }
}
