use std::collections::HashSet;
use ocl::{
    self,
    builders::KernelBuilder,
};
use uuid::Uuid;
use crate::{
    prelude::*,
    object::*,
    scene::{Scene, Background},
    Context,
    buffer::InstanceBuffer,
};


/// Scene with linear complexity of object search.
pub struct ListScene<O: Object, B: Background> {
    objects: Vec<O>,
    background: B,
    uuid: Uuid,
    max_depth: usize,
}

impl<O: Object, B: Background> ListScene<O, B> {
    pub fn new(background: B) -> Self {
        Self { objects: Vec::new(), background, uuid: Uuid::new_v4(), max_depth: 4 }
    }

    pub fn add(&mut self, object: O) {
        self.objects.push(object);
        self.uuid = Uuid::new_v4();
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }
    pub fn set_max_depth(&mut self, max_depth: usize) {
        self.max_depth = max_depth;
        self.uuid = Uuid::new_v4();
    }
}

impl<O: Object, B: Background> Scene for ListScene<O, B> {
    fn source(cache: &mut HashSet<u64>) -> String {
        [
            O::source(cache),
            B::source(cache),
            ObjectClass::methods().into_iter().map(|method| {
                format!(
                    "#define __object_{} {}_{}",
                    method, O::inst_name(), method,
                )
            }).collect::<Vec<_>>().join("\n"),
            format!("#define OBJECT_SIZE_INT {}", O::size_int()),
            format!("#define OBJECT_SIZE_FLOAT {}", O::size_float()),
            "#include <clay/scene/list_scene.h>".to_string(),
        ]
        .join("\n")
    }
}

pub struct ListSceneData<O: Object, B: Background> {
    buffer: InstanceBuffer<O>,
    background: B::Data,
    uuid: Uuid,
    max_depth: usize,
}

impl<O: Object, B: Background> Store for ListScene<O, B> {
    type Data = ListSceneData<O, B>;
    fn create_data(&self, context: &Context) -> clay_core::Result<Self::Data> {
        Ok(ListSceneData {
            buffer: InstanceBuffer::new(context, self.objects.iter())?,
            background: self.background.create_data(context)?,
            uuid: self.uuid, max_depth: self.max_depth,
        })
    }
    fn update_data(&self, context: &Context, data: &mut Self::Data) -> clay_core::Result<()> {
        if self.uuid != data.uuid {
            *data = self.create_data(context)?;
        }
        Ok(())
    }
}

impl<O: Object, B: Background> Push for ListSceneData<O, B> {
    fn args_def(kb: &mut KernelBuilder) {
        InstanceBuffer::<O>::args_def(kb);
        kb.arg(0i32);
        B::Data::args_def(kb);
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let mut j = i;
        self.buffer.args_set(j, k)?;
        j += InstanceBuffer::<O>::args_count();
        k.set_arg(j, &(self.max_depth as i32))?;
        j += 1;
        self.background.args_set(j, k)
    }
    fn args_count() -> usize {
        InstanceBuffer::<O>::args_count() +
        1 +
        B::Data::args_count()
    }
}
