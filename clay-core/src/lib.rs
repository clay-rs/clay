pub mod error; 
pub use error::Error;
pub mod result;
pub use result::Result;


pub mod push;
pub use push::*;
pub mod pack;
pub use pack::*;
pub mod type_hash;
pub use type_hash::*;

pub mod ray;
pub use ray::{Ray};
pub mod map;
pub use map::{Map, Mapper};
pub mod shape;
pub use shape::{Shape, Bound, Bounded};
pub mod material;
pub use material::{Material, Colored};
pub mod object;
pub use object::{Object, Covered};

pub mod scene;
pub use scene::*;
pub mod view;
pub use view::*;

pub mod ocl_src;
pub use ocl_src::{get_ocl_src};
pub mod context;
pub use context::{Context};
pub mod worker;
pub use worker::{Worker};
pub mod buffer;
pub use buffer::{Screen};
