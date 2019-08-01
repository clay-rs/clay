use crate::{pack::*, TypeHash, Shape};


pub trait Map: Pack + Sized + 'static {
    fn ocl_map_code() -> String;
    fn ocl_map_pref() -> String;
}

pub struct Mapper<S: Shape, M: Map> {
    pub shape: S,
    pub map: M,
}

impl<S: Shape, M: Map> Mapper<S, M> {
    pub fn new(shape: S, map: M) -> Self {
        Self { shape, map }
    }
}

impl<S: Shape, M: Map> Shape for Mapper<S, M> {
    fn ocl_shape_code() -> String {
        [
            S::ocl_shape_code(),
            M::ocl_map_code(),
            format!(
                "MAP_SHAPE_FN_DEF({}, {}, {}, {}, {})",
                Self::ocl_shape_fn(),
                S::ocl_shape_fn(),
                M::ocl_map_pref(),
                S::size_int(), S::size_float(),
            ),
        ].join("\n")
    }
    fn ocl_shape_fn() -> String {
        format!(
            "__mapper_{:x}__",
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