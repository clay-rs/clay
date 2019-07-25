use ocl::{
    self, 
    prm::*, 
    OclPrm, 
    builders::KernelBuilder,
};


pub trait Arg {
    fn def_arg<'b>(&'b self, kb: &mut KernelBuilder<'b>);
    fn set_arg(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()>;
    fn type_name(&self) -> String;
}

impl<T: Prm> Arg for T {
    fn def_arg<'b>(&'b self, kb: &mut KernelBuilder<'b>) {
        kb.arg(self);
    }
    fn set_arg(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        k.set_arg(i, self).map_err(|e| e.into())
    }
    fn type_name(&self) -> String {
        T::type_name()
    }
}

impl<T: Prm> Arg for Option<&ocl::Buffer<T>> {
    fn def_arg<'b>(&'b self, kb: &mut KernelBuilder<'b>) {
        kb.arg(*self);
    }
    fn set_arg(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        k.set_arg(i, *self).map_err(|e| e.into())
    }
    fn type_name(&self) -> String {
        ocl::Buffer::<T>::type_name()
    }
}


pub trait Prm: OclPrm + TypeName {}
pub trait TypeName {
    fn type_name() -> String {
        Self::type_name()
    }
}

impl<T: Prm> TypeName for ocl::Buffer<T> {
    fn type_name() -> String {
        format!("__global {} *", T::type_name())
    }
}
impl<T: Prm> TypeName for Option<&ocl::Buffer<T>> {
    fn type_name() -> String {
        ocl::Buffer::<T>::type_name()
    }
}

macro_rules! impl_prm {
    ($T:ident, $tn:expr) => {
        impl Prm for $T {}
        impl TypeName for $T {
            fn type_name() -> String {
                $tn.to_string()
            }
        }
    };
}

// Built-in types
impl_prm!(i8, "char");
impl_prm!(i16, "short");
impl_prm!(i32, "int");
impl_prm!(i64, "long");

impl_prm!(u8, "uchar");
impl_prm!(u16, "ushort");
impl_prm!(u32, "uint");
impl_prm!(u64, "ulong");

impl_prm!(f32, "float");
impl_prm!(f64, "double");

// OclPrm types
impl_prm!(Char, "char");
impl_prm!(Char2, "char2");
impl_prm!(Char3, "char3");
impl_prm!(Char4, "char4");
impl_prm!(Char8, "char8");
impl_prm!(Char16, "char16");

impl_prm!(Uchar, "uchar");
impl_prm!(Uchar2, "uchar2");
impl_prm!(Uchar3, "uchar3");
impl_prm!(Uchar4, "uchar4");
impl_prm!(Uchar8, "uchar8");
impl_prm!(Uchar16, "uchar16");

impl_prm!(Short, "short");
impl_prm!(Short2, "short2");
impl_prm!(Short3, "short3");
impl_prm!(Short4, "short4");
impl_prm!(Short8, "short8");
impl_prm!(Short16, "short16");

impl_prm!(Ushort, "ushort");
impl_prm!(Ushort2, "ushort2");
impl_prm!(Ushort3, "ushort3");
impl_prm!(Ushort4, "ushort4");
impl_prm!(Ushort8, "ushort8");
impl_prm!(Ushort16, "ushort16");

impl_prm!(Int, "int");
impl_prm!(Int2, "int2");
impl_prm!(Int3, "int3");
impl_prm!(Int4, "int4");
impl_prm!(Int8, "int8");
impl_prm!(Int16, "int16");

impl_prm!(Uint, "uint");
impl_prm!(Uint2, "uint2");
impl_prm!(Uint3, "uint3");
impl_prm!(Uint4, "uint4");
impl_prm!(Uint8, "uint8");
impl_prm!(Uint16, "uint16");

impl_prm!(Long, "long");
impl_prm!(Long2, "long2");
impl_prm!(Long3, "long3");
impl_prm!(Long4, "long4");
impl_prm!(Long8, "long8");
impl_prm!(Long16, "long16");

impl_prm!(Ulong, "ulong");
impl_prm!(Ulong2, "ulong2");
impl_prm!(Ulong3, "ulong3");
impl_prm!(Ulong4, "ulong4");
impl_prm!(Ulong8, "ulong8");
impl_prm!(Ulong16, "ulong16");

impl_prm!(Float, "float");
impl_prm!(Float2, "float2");
impl_prm!(Float3, "float3");
impl_prm!(Float4, "float4");
impl_prm!(Float8, "float8");
impl_prm!(Float16, "float16");

impl_prm!(Double, "double");
impl_prm!(Double2, "double2");
impl_prm!(Double3, "double3");
impl_prm!(Double4, "double4");
impl_prm!(Double8, "double8");
impl_prm!(Double16, "double16");
