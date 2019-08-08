use std::collections::HashSet;
use crate::Push;


pub trait View: Push {
    fn source(cache: &mut HashSet<u64>) -> String;
}
