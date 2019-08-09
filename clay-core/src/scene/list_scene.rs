use std::collections::HashSet;
use ocl::{
    self,
    builders::KernelBuilder,
};
use crate::{
    Context,
    Object,
    Attractor,
    buffer::InstanceBuffer,
};
use crate::{Push, Scene};

pub struct ListScene<T: Object, A: Attractor> {
    objects: Vec<T>,
    attractors: Vec<A>,
    object_buffer: InstanceBuffer<T>,
    attractor_buffer: InstanceBuffer<A>,
}

impl<T: Object, A: Attractor> ListScene<T, A> {
    pub fn new(context: &Context, objects: Vec<T>, attractors: Vec<A>) -> crate::Result<Self> {
        let object_buffer = InstanceBuffer::new(context, &objects)?;
        let attractor_buffer = InstanceBuffer::new(context, &attractors)?;
        Ok(Self { objects, attractors, object_buffer, attractor_buffer })
    }
}

impl<T: Object, A: Attractor> Scene for ListScene<T, A> {
    fn source(cache: &mut HashSet<u64>) -> String {
        // TODO: iterate over class methods
        [
            T::source(cache),
            A::source(cache),
            format!("#define __object_hit {}_hit", T::inst_name()),
            format!("#define __object_emit {}_emit", T::inst_name()),
            format!("#define __attract {}_attract", A::inst_name()),
            "#include <clay_core/scene/list_scene.h>".to_string(),
        ]
        .join("\n")
    }
}

impl<T: Object, A: Attractor> Push for ListScene<T, A> {
    fn args_def(kb: &mut KernelBuilder) {
        kb
        .arg(None::<&ocl::Buffer<i32>>)
        .arg(None::<&ocl::Buffer<f32>>)
        .arg(None::<&ocl::Buffer<i32>>)
        .arg(None::<&ocl::Buffer<f32>>)
        .arg(0i32)
        .arg(0i32)
        .arg(0i32)
        .arg(0i32)
        .arg(0i32)
        .arg(0i32);
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        k.set_arg(i + 0, self.object_buffer.buffer_int())?;
        k.set_arg(i + 1, self.object_buffer.buffer_float())?;
        k.set_arg(i + 2, self.attractor_buffer.buffer_int())?;
        k.set_arg(i + 3, self.attractor_buffer.buffer_float())?;
        k.set_arg(i + 4, T::size_int() as i32)?;
        k.set_arg(i + 5, T::size_float() as i32)?;
        k.set_arg(i + 6, A::size_int() as i32)?;
        k.set_arg(i + 7, A::size_float() as i32)?;
        k.set_arg(i + 8, self.objects.len() as i32)?;
        k.set_arg(i + 9, self.attractors.len() as i32)?;
        Ok(())
    }
    fn args_count() -> usize {
        10
    }
}
