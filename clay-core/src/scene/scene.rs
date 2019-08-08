use std::collections::HashSet;
use crate::Push;


pub trait Scene: Push {
    fn source(cache: &mut HashSet<u64>) -> String;
}
