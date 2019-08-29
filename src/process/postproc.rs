use std::ops::Deref;
use clay_core::{
    Context,
    filter::{IdentityFilter},
    process::{Postproc, PostprocBuilder},
};


pub struct DefaultPostproc {}

pub struct DefaultPostprocBuilder(
    PostprocBuilder<IdentityFilter>,
);

impl Deref for DefaultPostprocBuilder {
    type Target = PostprocBuilder<IdentityFilter>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DefaultPostproc {
    pub fn builder() -> crate::Result<DefaultPostprocBuilder> {
        let mut builder = Postproc::<IdentityFilter>::builder();
        builder.add_hook(crate::source());
        builder.collect().map(|b| DefaultPostprocBuilder(b))
    }
}

impl DefaultPostprocBuilder {
    pub fn build(self, context: &Context, dims: (usize, usize)) -> crate::Result<(Postproc<IdentityFilter>, String)> {
        self.0.build(context, dims, IdentityFilter::new())
    }
}
