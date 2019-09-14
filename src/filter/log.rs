use std::collections::HashSet;
use ocl::{self, builders::KernelBuilder};
use crate::{Push, filter::Filter};


pub struct LogFilter {
    lower: f64,
    upper: f64,
}

impl LogFilter {
    pub fn new(lower: f64, upper: f64) -> Self {
        Self { lower, upper }
    }
}

impl Filter for LogFilter {
    fn inst_name() -> String {
        "log_filter".to_string()
    }
    fn source(_: &mut HashSet<u64>) -> String {
        "#include <clay/filter/log.h>".to_string()
    }
}

impl Push for LogFilter {
    fn args_count() -> usize {
        2
    }
    fn args_def(kb: &mut KernelBuilder) {
        kb.arg(&0f32);
        kb.arg(&0f32);
    }
    fn args_set(&mut self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        k.set_arg(i + 0, &(self.lower as f32))?;
        k.set_arg(i + 1, &(self.upper as f32))?;
        Ok(())
    }
}
