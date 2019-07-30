use crate::Push;


pub trait View: Push {
    fn ocl_emit_code() -> String;
}
