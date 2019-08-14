use std::collections::HashSet;
use ocl::{
    self,
    builders::KernelBuilder,
};
use crate::{
    Context,
    Object,
    Target,
    buffer::InstanceBuffer,
};
use crate::{Push, Scene};

#[allow(dead_code)]
pub struct ListScene<O: Object, T: Target> {
    objects: Vec<O>,
    targets: Vec<T>,
    object_buffer: InstanceBuffer<O>,
    target_buffer: InstanceBuffer<T>,
}

impl<O: Object, T: Target> ListScene<O, T> {
    pub fn new(context: &Context, objects: Vec<O>, targets: Vec<T>) -> crate::Result<Self> {
        let object_buffer = InstanceBuffer::new(context, &objects)?;
        let target_buffer = InstanceBuffer::new(context, &targets)?;
        Ok(Self { objects, targets, object_buffer, target_buffer })
    }
}

impl<O: Object, T: Target> Scene for ListScene<O, T> {
    fn source(cache: &mut HashSet<u64>) -> String {
        // TODO: iterate over class methods
        [
            O::source(cache),
            T::source(cache),
            format!("#define __object_hit {}_hit", O::inst_name()),
            format!("#define __object_emit {}_emit", O::inst_name()),
            format!("#define __target_attract {}_attract", T::inst_name()),
            "#include <clay_core/scene/list_scene.h>".to_string(),
        ]
        .join("\n")
    }
}

impl<O: Object, T: Target> Push for ListScene<O, T> {
    fn args_def(kb: &mut KernelBuilder) {
        InstanceBuffer::<O>::args_def(kb);
        InstanceBuffer::<T>::args_def(kb);
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let mut j = i;
        self.object_buffer.args_set(j, k)?;
        j += InstanceBuffer::<O>::args_count();
        self.target_buffer.args_set(j, k)?;
        Ok(())
    }
    fn args_count() -> usize {
        InstanceBuffer::<O>::args_count() +
        InstanceBuffer::<T>::args_count()
    }
}
