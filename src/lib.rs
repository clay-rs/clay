pub use clay_core as core;


pub mod map;
pub mod shape;
pub mod material;

pub mod scene;
pub mod view;

pub mod filter;

pub mod process;

pub mod source;


pub mod prelude {
    pub use crate::core::prelude::*;
}

pub use clay_core::{Error, Result};

pub use clay_core::{
    object,
    context,
    buffer,
};

pub use prelude::*;
pub use context::*;
pub use source::*;

pub use clay_core::{
    select,
    shape_select,
    material_select,
    object_select,
    material_combine
};
