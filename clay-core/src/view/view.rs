use crate::Push;


pub trait View: Push {
    fn ocl_view_code() -> String;
}
