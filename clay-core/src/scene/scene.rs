use ocl::{
    self,
    builders::KernelBuilder,
};


pub trait Scene {
    fn ocl_trace_code() -> String;

    fn define_args(kb: &mut KernelBuilder);
    fn set_args(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()>;
}
