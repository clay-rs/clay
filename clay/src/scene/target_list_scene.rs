use std::collections::HashSet;
use ocl::{
    self,
    builders::KernelBuilder,
};
use clay_core::{
    Context,
    pack::*,
    class::*,
    shape::*,
    object::*,
    buffer::InstanceBuffer,
    Background,
};
use clay_core::{Push, Scene};

struct TargetData<T: Target> {
    object_index: usize,
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
        buffer_int.pack(&(self.object_index as i32));
        buffer_float.pack(&(self.brightness as f32));
        self.target.pack_to(
            &mut buffer_int[1..],
            &mut buffer_float[1..],
        );
    }
}

struct ObjectData<O: Object> {
    target_index: Option<usize>,
    object: O,
}

impl<O: Object> Pack for ObjectData<O> {
    fn size_int() -> usize {
        1 + O::size_int()
    }
    fn size_float() -> usize {
        O::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        buffer_int.pack(&(match self.target_index {
            Some(ti) => ti as i32,
            None => -1i32,
        }));
        self.object.pack_to(
            &mut buffer_int[1..],
            buffer_float,
        );
    }
}


type Element<O, T> = (O, Option<(T, f64)>);


#[allow(dead_code)]
pub struct TargetListSceneBuilder<O: Object + Targeted<T>, T: Target, B: Background> {
    elements: Vec<Element<O, T>>,
    background: B,
}

impl<O: Object + Targeted<T>, T: Target, B: Background> TargetListSceneBuilder<O, T, B> {
    pub fn add(&mut self, object: O) -> &mut Self {
        self.elements.push((object, None));
        self
    }
    pub fn add_targeted(&mut self, object: O) -> &mut Self {
        let target_opt = object.target();
        self.elements.push((object, target_opt));
        self
    }
    pub fn build(self, context: &Context) -> crate::Result<TargetListScene<O, T, B>> {
        TargetListScene::new(context, self.elements, self.background)
    }
}

pub struct TargetListScene<O: Object + Targeted<T>, T: Target, B: Background> {
    object_buffer: InstanceBuffer<ObjectData<O>>,
    target_buffer: InstanceBuffer<TargetData<T>>,
    background: B,
}

impl<O: Object + Targeted<T>, T: Target, B: Background> TargetListScene<O, T, B> {
    pub fn new(
        context: &Context,
        elements: Vec<Element<O, T>>,
        background: B,
    ) -> crate::Result<Self> {
        let mut objects = Vec::new();
        let mut targets = Vec::new();
        for (i, (object, target_opt)) in elements.into_iter().enumerate() {
            match target_opt {
                Some((target, brightness)) => {
                    objects.push(ObjectData {
                        target_index: Some(targets.len()),
                        object,
                    });
                    targets.push(TargetData {
                        object_index: i,
                        brightness, target,
                    });
                },
                None => {
                    objects.push(ObjectData {
                        target_index: None,
                        object,
                    });
                }
            }
        }
        let object_buffer = InstanceBuffer::new(context, &objects)?;
        let target_buffer = InstanceBuffer::new(context, &targets)?;
        Ok(Self { object_buffer, target_buffer, background })
    }

    pub fn builder(background: B) -> TargetListSceneBuilder<O, T, B> {
        TargetListSceneBuilder { elements: Vec::new(), background } 
    }
}

impl<O: Object + Targeted<T>, T: Target, B: Background> Scene for TargetListScene<O, T, B> {
    fn source(cache: &mut HashSet<u64>) -> String {
        // TODO: iterate over class methods
        [
            O::source(cache),
            T::source(cache),
            B::source(cache),
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
            "#include <clay/scene/target_list_scene.h>".to_string(),
        ]
        .join("\n")
    }
}

impl<O: Object + Targeted<T>, T: Target, B: Background> Push for TargetListScene<O, T, B> {
    fn args_def(kb: &mut KernelBuilder) {
        InstanceBuffer::<ObjectData<O>>::args_def(kb);
        InstanceBuffer::<TargetData<T>>::args_def(kb);
        B::args_def(kb);
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let mut j = i;
        self.object_buffer.args_set(j, k)?;
        j += InstanceBuffer::<ObjectData<O>>::args_count();
        self.target_buffer.args_set(j, k)?;
        j += InstanceBuffer::<TargetData<T>>::args_count();
        self.background.args_set(j, k)
    }
    fn args_count() -> usize {
        InstanceBuffer::<ObjectData<O>>::args_count() +
        InstanceBuffer::<TargetData<T>>::args_count() +
        B::args_count()
    }
}
