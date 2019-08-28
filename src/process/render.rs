use std::marker::PhantomData;
use clay_core::{
    Scene, View,
    process::{Renderer},
};


pub struct DefaultRenderer<S, V> {
    phantom: PhantomData<(S, V)>
}

impl<S: Scene, V: View> DefaultRenderer<S, V> {
    pub fn new(dims: (usize, usize), scene: S, view: V) -> crate::Result<Renderer<S, V>> {
        let mut builder = Renderer::<S, V>::builder();
        builder.add_hook(crate::source());
        builder.build(dims, scene, view)
    }
}
