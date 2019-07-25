use crate::worker::{Arg};

pub trait Scene {
    fn ocl_trace_code() -> String;

    fn args_def() -> Vec<Box<dyn Arg>>;
    fn args(&self) -> Vec<Box<dyn Arg>>;
}
