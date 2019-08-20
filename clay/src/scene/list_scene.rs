use std::collections::HashSet;
use ocl::{
    self,
    builders::KernelBuilder,
};
use clay_core::{
    Context,
    InstanceBuffer,
    class::*,
    object::*,
    Background,
};
use clay_core::{Push, Scene};


#[allow(dead_code)]
pub struct ListSceneBuilder<O: Object, B: Background> {
    objects: Vec<O>,
    background: B,
}

impl<O: Object, B: Background> ListSceneBuilder<O, B> {
    pub fn add(&mut self, object: O) -> &mut Self {
        self.objects.push(object);
        self
    }
    pub fn build(self, context: &Context) -> crate::Result<ListScene<O, B>> {
        ListScene::new(context, self.objects, self.background)
    }
}

pub struct ListScene<O: Object, B: Background> {
    buffer: InstanceBuffer<O>,
    background: B,
}

impl<O: Object, B: Background> ListScene<O, B> {
    pub fn new(
        context: &Context,
        objects: Vec<O>,
        background: B,
    ) -> crate::Result<Self> {
        let buffer = InstanceBuffer::new(context, &objects)?;
        Ok(Self { buffer, background })
    }

    pub fn builder(background: B) -> ListSceneBuilder<O, B> {
        ListSceneBuilder { objects: Vec::new(), background } 
    }
}

impl<O: Object, B: Background> Scene for ListScene<O, B> {
    fn source(cache: &mut HashSet<u64>) -> String {
        // TODO: iterate over class methods
        [
            O::source(cache),
            B::source(cache),
            ObjectClass::methods().into_iter().map(|method| {
                format!(
                    "#define __object_{} {}_{}",
                    method, O::inst_name(), method,
                )
            }).collect::<Vec<_>>().join("\n"),
            "#include <clay/scene/list_scene.h>".to_string(),
        ]
        .join("\n")
    }
}

impl<O: Object, B: Background> Push for ListScene<O, B> {
    fn args_def(kb: &mut KernelBuilder) {
        InstanceBuffer::<O>::args_def(kb);
        B::args_def(kb);
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let mut j = i;
        self.buffer.args_set(j, k)?;
        j += InstanceBuffer::<O>::args_count();
        self.background.args_set(j, k)
    }
    fn args_count() -> usize {
        InstanceBuffer::<O>::args_count() +
        B::args_count()
    }
}
