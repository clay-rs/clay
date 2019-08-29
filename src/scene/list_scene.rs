use std::collections::HashSet;
use ocl::{
    self,
    builders::KernelBuilder,
};
use uuid::Uuid;
use clay_core::{
    Context,
    InstanceBuffer,
    class::*,
    object::*,
    Background,
};
use clay_core::{Push, Store, Scene};


#[allow(dead_code)]
pub struct ListScene<O: Object, B: Background> {
    objects: Vec<O>,
    background: B,
    uuid: Uuid,
}

impl<O: Object, B: Background> ListScene<O, B> {
    pub fn new(background: B) -> Self {
        Self { objects: Vec::new(), background, uuid: Uuid::new_v4() }
    }

    pub fn add(&mut self, object: O) {
        self.objects.push(object);
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
}

impl<O: Object, B: Background> Store for ListScene<O, B> {
    type Data = ListSceneData<O, B>;
    fn create_data(&self, context: &Context) -> clay_core::Result<Self::Data> {
        Ok(ListSceneData {
            buffer: InstanceBuffer::new(context, self.objects.iter())?,
            background: self.background.create_data(context)?,
            uuid: self.uuid,
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
        B::Data::args_def(kb);
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let mut j = i;
        self.buffer.args_set(j, k)?;
        j += InstanceBuffer::<O>::args_count();
        self.background.args_set(j, k)
    }
    fn args_count() -> usize {
        InstanceBuffer::<O>::args_count() +
        B::Data::args_count()
    }
}
