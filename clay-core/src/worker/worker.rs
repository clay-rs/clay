use vecmat::{
    vec::*,
    mat::*,
};
use crate::{Screen};


pub trait Worker {
    fn render(
        &self,
        screen: &mut Screen,
        pos: Vec3<f64>,
        map: Mat3<f64>,
    ) -> crate::Result<()>;
}
