use crate::Push;


pub trait Scene: Push {
    fn ocl_scene_code() -> String;
}
