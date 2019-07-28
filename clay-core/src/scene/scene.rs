use crate::Push;


pub trait Scene: Push {
    fn ocl_trace_code() -> String;
}
