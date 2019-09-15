use crate::{scene::Scene, view::View};
pub use crate::core::process::{
    RendererBuilder, Renderer,
    RenderWorker, RenderData,
};

/// Creates renderer with already included device source from `clay` and `clay-core`.
pub fn create_renderer<S: Scene, V: View>() -> RendererBuilder<S, V> {
    let mut builder = crate::core::process::create_renderer::<S, V>();
    builder.add_hook(crate::source());
    builder
}
