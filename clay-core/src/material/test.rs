use std::{
    collections::HashSet,
    marker::PhantomData,
};
use crate::{pack::*, class::*, material::*};


#[derive(Clone, Debug, Default)]
pub struct TestMaterial<T: 'static> {
    phantom: PhantomData<T>,
}

impl<T> TestMaterial<T> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}

impl<T> Material for TestMaterial<T> {
    fn brightness(&self) -> f64 {
        0.0
    }
}

impl<T> Instance<MaterialClass> for TestMaterial<T> {
    fn source(_: &mut HashSet<u64>) -> String {
        String::new()
    }
    fn inst_name() -> String {
        "test_material".to_string()
    }
}

impl<T> Pack for TestMaterial<T> {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
