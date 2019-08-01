use crate::{Pack};


/// An abstract object that could be drawn completely.
pub trait Object: Pack + Sized + 'static {
    fn ocl_object_code() -> String;
    fn ocl_object_fn() -> (String, String);
}
