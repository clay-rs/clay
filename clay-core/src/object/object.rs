use crate::{Pack, Shape, Material};

/// An abstract object that could be drawn completely.
pub trait Object: Pack + Shape + Material {
    
}
