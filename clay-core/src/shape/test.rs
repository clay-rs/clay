use std::{
    collections::HashSet,
    marker::PhantomData,
};
use crate::{
    pack::*,
    class::*,
    shape::*,
};

#[derive(Clone, Debug, Default)]
pub struct TestShape<T: 'static> {
    phantom: PhantomData<T>,
}

impl<T> TestShape<T> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}

impl<T> Shape for TestShape<T> {}

impl<T> Instance<ShapeClass> for TestShape<T> {
    fn source(_: &mut HashSet<u64>) -> String { String::new() }
    fn inst_name() -> String { "test_shape".to_string() }
}

impl<T> Pack for TestShape<T> {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 0 }
    fn pack_to(&self, _buffer_int: &mut [i32], _buffer_float: &mut [f32]) {}
}
