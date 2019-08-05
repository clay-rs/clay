use vecmat::vec::*;
use crate::{
    pack::*, 
    Material,
    TypeHash,
};


#[derive(Clone, Debug, Default)]
pub struct Colored<M: Material> {
    pub material: M,
    pub color: Vec3<f64>,
}

impl<M: Material> Colored<M> {
    pub fn new(material: M, color: Vec3<f64>) -> Self {
        Self { material, color }
    }

    fn ocl_fn() -> String {
        format!(
            "__{}_colored_{:x}__",
            M::ocl_material_fn(),
            Self::type_hash(),
        )
    }
}

impl<M: Material> Material for Colored<M> {
    fn ocl_material_code() -> String {
        [
            M::ocl_material_code(),
            "#include <clay_core/material/colored.h>".to_string(),
            format!(
                "__COLORED_MATERIAL_FN_DEF__({}, {}, {}, {})",
                Self::ocl_fn(),
                M::ocl_material_fn(),
                M::size_int(),
                M::size_float(),
            ),
        ].join("\n")
    }
    fn ocl_material_fn() -> String {
        Self::ocl_fn()
    }
}

impl<M: Material> Pack for Colored<M> {
    fn size_int() -> usize { M::size_int() + 0 }
    fn size_float() -> usize { M::size_float() + 3 }

    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        self.material.pack_to(buffer_int, buffer_float);
        self.color.pack_float_to(&mut buffer_float[M::size_float()..]);
    }
}
