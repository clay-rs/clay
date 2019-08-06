use crate::{Pack, Packer, TypeHash, Shape, Material, Object};


#[derive(Clone, Debug, Default)]
/// Object obtained by covering shape with material
pub struct Covered<S: Shape, M: Material> {
    pub shape: S,
    pub material: M,
}

impl<S: Shape, M: Material> Covered<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Self { shape, material }
    }
}

impl<S: Shape, M: Material> Object for Covered<S, M> {
    fn source() -> String {
        [
            S::source(),
            M::source(),
            [
                &format!(
                    "#define {}_hit {}_hit",
                    Self::instance(),
                    S::instance(),
                ),
                "",
                &format!("MATERIAL_RET {}_emit(", Self::instance()),
                "\tMATERIAL_ARGS_DEF",
                ") {",
                &format!(
                    "\treturn {}_emit(MATERIAL_ARGS_B({}, {}));",
                    M::instance(), S::size_int(), S::size_float(),
                ),
                "}",
            ].join("\n")
        ].join("\n")
    }
    fn instance() -> String {
        format!("__covered_{:x}", Self::type_hash())
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
