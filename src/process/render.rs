use std::marker::PhantomData;
use crate::{
    scene::Scene, view::View,
};

pub use crate::core::process::{
    RendererBuilder, Renderer as CoreRenderer,
    RenderData, RenderWorker,
};


pub struct Renderer<S, V> {
    phantom: PhantomData<(S, V)>
}

impl<S: Scene, V: View> Renderer<S, V> {
    pub fn builder() -> RendererBuilder<S, V> {
        let mut builder = CoreRenderer::<S, V>::builder();
        builder.add_hook(crate::source());
        builder
    }
}
