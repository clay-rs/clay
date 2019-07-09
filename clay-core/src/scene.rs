use crate::object::Object;

#[macro_export]
macro_rules! combine_objects {
    () => {};
}

pub struct Scene<T: Object> {
    objs: Vec<T>,
}

impl<T: Object> Scene<T> {

}
