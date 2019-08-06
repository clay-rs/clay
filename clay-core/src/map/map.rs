use crate::{
    pack::*,
    class::*,
};


pub trait Map: Pack + Instance<MapClass> + Sized + 'static {}

pub enum MapClass {}
impl Class for MapClass {
    fn name() -> String {
        "map".to_string()
    }
    fn methods() -> Vec<String> {
        [
            "rel",
            "abs",
            "rel_inv",
            "abs_inv",
            "norm",
        ]
        .iter()
        .map(|m| m.to_string())
        .collect()
    }
}
