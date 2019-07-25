use ocl::{self, prm};
use crate::{
    shape::Shape,
    worker::Arg,
};
use super::Scene;


pub struct ListScene<T: Shape> {
    objects: Vec<T>,
}

impl<T: Shape> Scene for ListScene<T> {
    fn args_def() -> Vec<Box<dyn Arg>> {
        vec![
            Box::new(None::<&ocl::Buffer<i32>>),
            Box::new(None::<&ocl::Buffer<f32>>),
            Box::new(0i32),
            Box::new(0i32),
            Box::new(0i32),
        ]
    }
    fn args(&self) -> Vec<Box<dyn Arg>> {
        vec![
            Box::new(None::<&ocl::Buffer<i32>>),
            Box::new(None::<&ocl::Buffer<f32>>),
            Box::new(0i32),
            Box::new(0i32),
            Box::new(0i32),
        ]
    }
}

