use std::collections::HashSet;
use crate::{pack::*, class::*, TypeHash, Map, object::*};


pub struct Mapper<O: Object, M: Map> {
    pub object: O,
    pub map: M,
}

impl<O: Object, M: Map> Mapper<O, M> {
    pub fn new(object: O, map: M) -> Self {
        Self { object, map }
    }
}

impl<O: Object, M: Map> Object for Mapper<O, M> {}

impl<O: Object, M: Map> Instance<ObjectClass> for Mapper<O, M> {
    fn source(cache: &mut HashSet<u64>) -> String {
        if !cache.insert(Self::type_hash()) {
            return String::new()
        }
        [
            O::source(cache),
            M::source(cache),
            format!(
                "MAP_OBJECT_FN_DEF({}, {}, {}, {}, {})",
                Self::inst_name(),
                O::inst_name(),
                M::inst_name(),
                O::size_int(), O::size_float(),
            ),
            format!(
                "#define {}_emit {}_emit",
                Self::inst_name(),
                O::inst_name(),
            ),
        ].join("\n")
    }
    fn inst_name() -> String {
        format!(
            "__mapper_{:x}",
            Self::type_hash(),
        )
    }
}


impl<O: Object, M: Map> Pack for Mapper<O, M> {
    fn size_int() -> usize {
        O::size_int() + M::size_int()
    }
    fn size_float() -> usize {
        O::size_float() + M::size_float()
    }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        Packer::new(buffer_int, buffer_float)
        .pack(&self.object)
        .pack(&self.map);
    }
}
