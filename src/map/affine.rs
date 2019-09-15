use crate::map::*;

/// Affine transformation.
pub type Affine = Chain<Linear, Shift>;
