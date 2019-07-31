use std::{
    hash::{Hash, Hasher},
    any::TypeId,
    collections::hash_map::DefaultHasher,
};
use crate::{Pack, Packer, Shape, Material, Object};


#[derive(Clone, Debug, Default)]
/// Object obtained by covering shape with material
pub struct Covered<S: Shape + 'static, M: Material + 'static> {
    pub shape: S,
    pub material: M,
}

impl<S: Shape + 'static, M: Material + 'static> Covered<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Self { shape, material }
    }

    fn type_hash() -> u64 {
        let mut hasher = DefaultHasher::new();
        TypeId::of::<Self>().hash(&mut hasher);
        hasher.finish()
    }

    fn ocl_shape_fn() -> String {
        S::ocl_shape_fn()
    }

    fn ocl_material_fn() -> String {
        format!(
            "__{}__{:x}__",
            M::ocl_material_fn(),
            Self::type_hash(),
        )
    }
}

impl<S: Shape + 'static, M: Material + 'static> Object for Covered<S, M> {
    fn ocl_object_code() -> String {
        [
            S::ocl_shape_code(),
            M::ocl_material_code(),
            [
                &format!("int {}(", Self::ocl_material_fn()),
                "\nRay r,",
                "\nfloat3 p, float3 n,",
                "\n__global const int *ibuf,",
                "\n__global const float *fbuf,",
                "\nRay *rr, float3 *glow",
                ") {",
                &format!(
                    "\treturn {}(r, p, n, ibuf + {}, fbuf + {}, rr, glow);",
                    M::ocl_material_fn(), S::size_int(), S::size_float(),
                ),
                "}",
            ].join("\n")
        ].join("\n")
    }
    fn ocl_object_fn() -> (String, String) {
        (Self::ocl_shape_fn(), Self::ocl_material_fn())
    }
}

impl<S: Shape, M: Material> Pack for Covered<S, M> {
    fn size_int() -> usize {
        S::size_int() + M::size_int()
    }
    fn size_float() -> usize {
        S::size_float() + M::size_float()
    }

    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        Packer::new(buffer_int, buffer_float)
        .pack(&self.shape)
        .pack(&self.material);
    }
}
