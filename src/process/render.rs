use crate::{scene::Scene, view::View};
pub use crate::core::process::{
    RendererBuilder, Renderer,
    RenderWorker, RenderData,
};

pub fn create_renderer<S: Scene, V: View>() -> RendererBuilder<S, V> {
    let mut builder = crate::core::process::create_renderer::<S, V>();
    builder.add_hook(crate::source());
    builder
}
