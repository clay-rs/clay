use clay_core::{
    Context,
    filter::{IdentityFilter},
    process::{Postproc},
};


pub struct DefaultPostproc {}

impl DefaultPostproc {
    pub fn new(
        context: &Context, dims: (usize, usize),
    ) -> crate::Result<(Postproc<IdentityFilter>, String)> {
        let mut builder = Postproc::<IdentityFilter>::builder();
        builder.add_hook(crate::source());
        let builder = builder.collect()?;
        builder.build(context, dims, IdentityFilter::new())
    }
}
