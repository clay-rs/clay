use crate::{Pack, Geometry, Material};


/// An abstract object that could be drawn completely.
pub trait Object: Pack + Geometry + Material {}
