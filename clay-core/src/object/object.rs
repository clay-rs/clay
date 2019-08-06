use crate::{Pack};


/// An abstract object that could be drawn completely.
pub trait Object: Pack + Sized + 'static {
    fn source() -> String;
    fn instance() -> String;
    fn class() -> String {
    	"object".to_string()
    }
    fn methods() -> Vec<String> {
    	vec![
    	"hit".to_string(),
    	"emit".to_string(),
    	]
    }
}
