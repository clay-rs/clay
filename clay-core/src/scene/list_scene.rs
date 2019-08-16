use std::collections::HashSet;
use ocl::{
    self,
    builders::KernelBuilder,
};
use crate::{
    Context,
    pack::*,
    class::*,
    shape::*,
    object::*,
    buffer::InstanceBuffer,
};
use crate::{Push, Scene};


struct TargetData<T> {
    index: usize,
    brightness: f64,
    target: T,
}

impl<T: Target> Pack for TargetData<T> {
    fn size_int() -> usize {
        1 + T::size_int()
    }
    fn size_float() -> usize {
        1 + T::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        buffer_int.pack(&(self.index as i32));
        buffer_float.pack(&(self.brightness as f32));
        self.target.pack_to(
            &mut buffer_int[1..],
            &mut buffer_float[1..],
        );
    }
}

type Element<O, T> = (O, Option<(T, f64)>);


#[allow(dead_code)]
pub struct ListSceneBuilder<O: Object + Targeted<T>, T: Target> {
    elements: Vec<Element<O, T>>,
}

impl<O: Object + Targeted<T>, T: Target> ListSceneBuilder<O, T> {
    pub fn add(&mut self, object: O) -> &mut Self {
        self.elements.push((object, None));
        self
    }
    pub fn add_targeted(&mut self, object: O) -> &mut Self {
        let target_opt = object.target();
        self.elements.push((object, target_opt));
        self
    }
    pub fn build(self, context: &Context) -> crate::Result<ListScene<O, T>> {
        ListScene::new(context, self.elements)
    }
}

pub struct ListScene<O: Object, T: Target> {
    object_buffer: InstanceBuffer<O>,
    target_buffer: InstanceBuffer<TargetData<T>>,
}

impl<O: Object + Targeted<T>, T: Target> ListScene<O, T> {
    pub fn new(context: &Context, elements: Vec<Element<O, T>>) -> crate::Result<Self> {
        let mut objects = Vec::new();
        let mut targets = Vec::new();
        for (i, (object, target_opt)) in elements.into_iter().enumerate() {
            objects.push(object);
            if let Some((target, brightness)) = target_opt {
                targets.push(TargetData { index: i, brightness, target });
            }
        }
        let object_buffer = InstanceBuffer::new(context, &objects)?;
        let target_buffer = InstanceBuffer::new(context, &targets)?;
        Ok(Self { object_buffer, target_buffer })
    }

    pub fn builder() -> ListSceneBuilder<O, T> {
        ListSceneBuilder { elements: Vec::new() } 
    }
}

impl<O: Object + Targeted<T>, T: Target> Scene for ListScene<O, T> {
    fn source(cache: &mut HashSet<u64>) -> String {
        // TODO: iterate over class methods
        [
            O::source(cache),
            T::source(cache),
            ObjectClass::methods().into_iter().map(|method| {
                format!(
                    "#define __object_{} {}_{}",
                    method, O::inst_name(), method,
                )
            }).collect::<Vec<_>>().join("\n"),
            TargetClass::methods().into_iter().map(|method| {
                format!(
                    "#define __target_{} {}_{}",
                    method, T::inst_name(), method,
                )
            }).collect::<Vec<_>>().join("\n"),
            "#include <clay_core/scene/list_scene.h>".to_string(),
        ]
        .join("\n")
    }
}

impl<O: Object + Targeted<T>, T: Target> Push for ListScene<O, T> {
    fn args_def(kb: &mut KernelBuilder) {
        InstanceBuffer::<O>::args_def(kb);
        InstanceBuffer::<TargetData<T>>::args_def(kb);
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
        InstanceBuffer::<TargetData<T>>::args_count()
    }
}
