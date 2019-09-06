use crate::{
    filter::{Filter, IdentityFilter},
};
pub use crate::core::process::{
    PostprocBuilder, PostprocCollector, Postproc,
};

pub fn create_postproc<F: Filter>() -> PostprocCollector<F> {
    let mut collector = crate::core::process::create_postproc::<F>();
    collector.add_hook(crate::source());
    collector
}

pub fn create_default_postproc() -> PostprocCollector<IdentityFilter> {
    create_postproc::<IdentityFilter>()
}
