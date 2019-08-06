use vecmat::{vec::*};
use crate::{Map};


pub type Shift = Vec3<f64>;

impl Map for Shift {
    fn source() -> String {
        "#include <clay_core/map/shift.h>".to_string()
    }
    fn instance() -> String {
        "shift".to_string()
    }
}
