use std::marker::PhantomData;
use clay_core::{Scene, View, worker::*};

pub struct DefaultWorker<S, V> {
    phantom: PhantomData<(S, V)>
}

impl<S: Scene, V: View> DefaultWorker<S, V> {
    pub fn builder() -> crate::Result<WorkerBuilder<S, V>> {
        let mut builder = Worker::<S, V>::builder();
        builder.add_hook(crate::source());
        builder.collect()
    }
}
