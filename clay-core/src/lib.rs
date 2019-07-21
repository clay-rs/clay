pub mod error; 
pub mod result;

mod pack;

mod ray;
pub mod geometry;
pub mod material;
//pub mod object;

//mod scene;


mod context;
mod worker;
pub mod buffer;

pub use error::Error;
pub use result::Result;


pub use pack::Pack;
pub use ray::Ray;

pub use geometry::{Geometry, Bound, Bounded};
pub use material::Material;
//pub use object::Object;

//pub use scene::Scene;


pub use context::Context;
pub use worker::Worker;
pub use buffer::Screen;
