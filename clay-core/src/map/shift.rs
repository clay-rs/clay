use vecmat::{vec::*};
use crate::{class::*, map::*};


pub type Shift = Vec3<f64>;

impl Map for Shift {}

impl Instance<MapClass> for Shift {
    fn source() -> String {
        "#include <clay_core/map/shift.h>".to_string()
    }
    fn inst_name() -> String {
        "shift".to_string()
    }
}
