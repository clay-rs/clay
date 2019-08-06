use crate::{
    pack::*,
    declare_callable,
};


pub trait Map: Pack + Sized + 'static {
    declare_callable!(
        "map".to_string(),
        [
            "rel",
            "abs",
            "rel_inv",
            "abs_inv",
            "norm",
        ]
        .iter()
        .map(|m| m.to_string())
        .collect(),
    );
}
