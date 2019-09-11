use std::{
    rc::Rc,
    cell::Cell,
    collections::HashSet,
};
use ocl::{
    self,
    builders::KernelBuilder,
};
use uuid::Uuid;
use crate::{
    prelude::*,
    shape::*,
    object::*,
    scene::{Scene, Background},
    Context,
    buffer::InstanceBuffer,
};


struct TargetData<T: Target> {
    object_index: usize,
    brightness: f64,
    target: Rc<T>,
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
    object: Rc<O>,
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


/// Scene with linear complexity and importance sampling of bright objects.
pub struct TargetListScene<O: Object + Targeted<T>, T: Target, B: Background> {
    elements: Cell<Vec<Element<O, T>>>,
    background: B,
    uuid: Uuid,
    max_depth: usize,
}

impl<O: Object + Targeted<T>, T: Target, B: Background> TargetListScene<O, T, B> {
    pub fn new(background: B) -> Self {
        Self { elements: Cell::new(Vec::new()), background, uuid: Uuid::new_v4(), max_depth: 4 } 
    }
    pub fn add(&mut self, object: O) {
        self.elements.get_mut().push((object, None));
        self.uuid = Uuid::new_v4();
    }
    pub fn add_targeted(&mut self, object: O) {
        let target_opt = object.target();
        self.elements.get_mut().push((object, target_opt));
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

pub struct TargetListSceneData<O: Object + Targeted<T>, T: Target, B: Background> {
    object_buffer: InstanceBuffer<ObjectData<O>>,
    target_buffer: InstanceBuffer<TargetData<T>>,
    background: B::Data,
    uuid: Uuid,
    max_depth: usize,
}

impl<O: Object + Targeted<T>, T: Target, B: Background> Scene for TargetListScene<O, T, B> {
    fn source(cache: &mut HashSet<u64>) -> String {
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
            format!("#define OBJECT_SIZE_INT {}", ObjectData::<O>::size_int()),
            format!("#define OBJECT_SIZE_FLOAT {}", ObjectData::<O>::size_float()),
            format!("#define TARGET_SIZE_INT {}", TargetData::<T>::size_int()),
            format!("#define TARGET_SIZE_FLOAT {}", TargetData::<T>::size_float()),
            "#include <clay/scene/target_list_scene.h>".to_string(),
        ]
        .join("\n")
    }
}

impl<O: Object + Targeted<T>, T: Target, B: Background> Store for TargetListScene<O, T, B> {
    type Data = TargetListSceneData<O, T, B>;
    fn create_data(&self, context: &Context) -> clay_core::Result<Self::Data> {
        let elems = self.elements.replace(Vec::new())
        .into_iter().map(|(o, to)| {
            (Rc::new(o), to.map(|t| (Rc::new(t.0), t.1)))
        }).collect::<Vec<_>>();

        let mut objects = Vec::new();
        let mut targets = Vec::new();
        for (i, (object, target_opt)) in elems.iter().enumerate() {
            match target_opt {
                Some((target, brightness)) => {
                    objects.push(ObjectData {
                        target_index: Some(targets.len()),
                        object: object.clone(),
                    });
                    targets.push(TargetData {
                        object_index: i,
                        target: target.clone(),
                        brightness: *brightness,
                    });
                },
                None => {
                    objects.push(ObjectData {
                        target_index: None,
                        object: object.clone(),
                    });
                }
            }
        }

        let res = InstanceBuffer::new(context, objects.iter())
        .and_then(|ob| InstanceBuffer::new(context, targets.iter()).map(|tb| (ob, tb)));
        let _ = (objects, targets);

        assert_eq!(self.elements.replace(
            elems.into_iter().map(|(o, to)| {
                (
                    Rc::try_unwrap(o).map_err(|_| "Rc still exists somewhere").unwrap(),
                    to.map(|t| (
                        Rc::try_unwrap(t.0).map_err(|_| "Rc still exists somewhere").unwrap(),
                        t.1,
                    )),
                )
            }).collect::<Vec<_>>()
        ).len(), 0);
        
        let (object_buffer, target_buffer) = res?;

        Ok(Self::Data {
            object_buffer, target_buffer,
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

impl<O: Object + Targeted<T>, T: Target, B: Background> Push for TargetListSceneData<O, T, B> {
    fn args_def(kb: &mut KernelBuilder) {
        InstanceBuffer::<ObjectData<O>>::args_def(kb);
        InstanceBuffer::<TargetData<T>>::args_def(kb);
        kb.arg(0i32);
        B::Data::args_def(kb);
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        let mut j = i;
        self.object_buffer.args_set(j, k)?;
        j += InstanceBuffer::<ObjectData<O>>::args_count();
        self.target_buffer.args_set(j, k)?;
        j += InstanceBuffer::<TargetData<T>>::args_count();
        k.set_arg(j, &(self.max_depth as i32))?;
        j += 1;
        self.background.args_set(j, k)
    }
    fn args_count() -> usize {
        InstanceBuffer::<ObjectData<O>>::args_count() +
        InstanceBuffer::<TargetData<T>>::args_count() +
        1 +
        B::Data::args_count()
    }
}
