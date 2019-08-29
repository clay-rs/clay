use std::{
    ops::Deref,
    marker::PhantomData,
};
use clay_core::{
    Context,
    filter::{Filter, IdentityFilter},
    process as core_proc,
};


pub struct Postproc<F: Filter> {
    phantom: PhantomData<F>,
}

pub struct PostprocBuilder<F: Filter>(
    core_proc::PostprocBuilder<F>,
);

impl<F: Filter> Deref for PostprocBuilder<F> {
    type Target = core_proc::PostprocBuilder<F>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Filter> Postproc<F> {
    pub fn builder() -> crate::Result<PostprocBuilder<F>> {
        let mut builder = core_proc::Postproc::<F>::builder();
        builder.add_hook(crate::source());
        builder.collect().map(|b| PostprocBuilder(b))
    }
}

impl<F: Filter> PostprocBuilder<F> {
    pub fn build(self, context: &Context, dims: (usize, usize), filter: F) -> crate::Result<(core_proc::Postproc<F>, String)> {
        self.0.build(context, dims, filter)
    }
}

impl<F: Filter + Default> PostprocBuilder<F> {
    pub fn build_default(self, context: &Context, dims: (usize, usize)) -> crate::Result<(core_proc::Postproc<F>, String)> {
        self.0.build(context, dims, F::default())
    }
}

pub type DefaultPostproc = Postproc<IdentityFilter>;
