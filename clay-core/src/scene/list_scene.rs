use ocl::{
    self,
    builders::KernelBuilder,
};
use crate::{
    Context,
    Shape,
    buffer::ObjectBuffer,
};
use crate::{Push, Scene};

pub struct ListScene<T: Shape> {
    objects: Vec<T>,
    buffer: ObjectBuffer<T>,
}

impl<T: Shape> ListScene<T> {
    pub fn new(objects: Vec<T>, context: &Context) -> crate::Result<Self> {
        let buffer = ObjectBuffer::new(context, &objects)?;
        Ok(Self { objects, buffer })
    }
}

impl<T: Shape> Scene for ListScene<T> {
    fn ocl_trace_code() -> String {
        format!("{}\n{}\n{}",
            T::ocl_hit_code(),
            format!("#define hit {}", T::ocl_hit_fn()), 
            "#include <scene.h>",
        )
    }
}

impl<T: Shape> Push for ListScene<T> {
    fn args_def(kb: &mut KernelBuilder) {
        kb
        .arg(None::<&ocl::Buffer<i32>>)
        .arg(None::<&ocl::Buffer<f32>>)
        .arg(0i32)
        .arg(0i32)
        .arg(0i32);
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        k.set_arg(i + 0, self.buffer.buffer_int())?;
        k.set_arg(i + 1, self.buffer.buffer_float())?;
        k.set_arg(i + 2, T::size_int() as i32)?;
        k.set_arg(i + 3, T::size_float() as i32)?;
        k.set_arg(i + 4, self.objects.len() as i32)?;
        Ok(())
    }
}
