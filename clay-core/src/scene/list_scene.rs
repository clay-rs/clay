use ocl::{
    self,
    builders::KernelBuilder,
};
use crate::{
    Context,
    Object,
    buffer::ObjectBuffer,
};
use crate::{Push, Scene};

pub struct ListScene<T: Object> {
    objects: Vec<T>,
    buffer: ObjectBuffer<T>,
}

impl<T: Object> ListScene<T> {
    pub fn new(objects: Vec<T>, context: &Context) -> crate::Result<Self> {
        let buffer = ObjectBuffer::new(context, &objects)?;
        Ok(Self { objects, buffer })
    }
}

impl<T: Object> Scene for ListScene<T> {
    fn source() -> String {
        [
            T::source(),
            format!("#define __object_hit {}_hit", T::instance()),
            format!("#define __object_emit {}_emit", T::instance()),
            "#include <clay_core/scene/list_scene.h>".to_string(),
        ]
        .join("\n")
    }
}

impl<T: Object> Push for ListScene<T> {
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
    fn args_count() -> usize {
        5
    }
}
