use crate::{
    Pack,
    class::*,
};


/// An abstract object that could be drawn completely.
pub trait Object: Pack + Instance<ObjectClass> {}

pub enum ObjectClass {}
impl Class for ObjectClass {
    fn name() -> String {
        "object".to_string()
    }
    fn methods() -> Vec<String> {
        vec![
            "hit".to_string(),
            "emit".to_string(),
        ]
    }
}
