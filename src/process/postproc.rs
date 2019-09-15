use crate::{
    filter::{Filter, IdentityFilter},
};
pub use crate::core::process::{
    PostprocBuilder, PostprocCollector, Postproc,
};

/// Creates postrocessor with already included device source from `clay` and `clay-core`.
pub fn create_postproc<F: Filter>() -> PostprocCollector<F> {
    let mut collector = crate::core::process::create_postproc::<F>();
    collector.add_hook(crate::source());
    collector
}

/// Creates postrocessor with identity filter.
pub fn create_default_postproc() -> PostprocCollector<IdentityFilter> {
    create_postproc::<IdentityFilter>()
}
