use crate::{pack::*, TypeHash, Shape};


pub trait Map: Pack {
    fn ocl_map_code() -> String;
    fn ocl_map_pref() -> String;
}

pub struct Mapper<S: Shape, M: Map> {
    pub shape: S,
    pub map: M,
}

impl<S: Shape + 'static, M: Map + 'static> Shape for Mapper<S, M> {
    
}

impl<S: Shape + 'static, M: Map + 'static> Shape for Mapper<S, M> {
    fn ocl_shape_code() -> String {
        [
            S::ocl_shape_code(),
            M::ocl_map_code(),
            [
                &format!("__SHAPE_RET__ {}(", Self::ocl_shape_fn()),
                "\t__SHAPE_ARGS_DEF__",
                ") {"
            ].join("\n")
        ].join("\n")
    }
    fn ocl_shape_pref() -> String {
        format!(
            "__{}_{:x}__",
            S::ocl_shape_fn(),
            Self::type_hash(),
        )
    }
}

impl<S: Shape, M: Map> Pack for Mapper<S, M> {
    fn size_int() -> usize {
        S::size_int() + M::size_int()
    }
    fn size_float() -> usize {
        S::size_float() + M::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        Packer::new(buffer_int, buffer_float)
        .pack(&self.shape)
        .pack(&self.map);
    }
}