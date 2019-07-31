use std::{
    hash::{Hash, Hasher},
    any::TypeId,
    collections::hash_map::DefaultHasher,
};

pub trait TypeHash: 'static {
    fn type_hash() -> u64 {
        let mut hasher = DefaultHasher::new();
        TypeId::of::<Self>().hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: 'static> TypeHash for T {}
