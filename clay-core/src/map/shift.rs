use vecmat::{vec::*};
use crate::{Map};


pub type Shift = Vec3<f64>;

impl Map for Shift {
    fn ocl_map_code() -> String {
        "#include <map/shift.h>".to_string()
    }
    fn ocl_map_pref() -> String {
        "shift".to_string()
    }
}
