use ocl::{
    self,
    builders::KernelBuilder,
};

/// Something that could be pushed to OpenCL kernel as argumets
pub trait Push {
    fn args_def(kb: &mut KernelBuilder);
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()>;
    fn args_count() -> usize;
}
