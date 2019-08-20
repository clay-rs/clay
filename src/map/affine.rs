use clay_core::Chain;
use super::{Linear, Shift};

pub type Affine = Chain<Linear, Shift>;
