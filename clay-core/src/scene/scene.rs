use crate::Push;


pub trait Scene: Push {
    fn source() -> String;
}
