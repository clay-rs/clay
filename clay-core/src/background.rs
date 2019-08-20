use std::collections::HashSet;
use crate::Push;


/// Background of the scene.
pub trait Background: Push {
    fn source(cache: &mut HashSet<u64>) -> String;
}
