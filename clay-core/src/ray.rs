use vecmat::vec::*;


pub struct Ray {
    pub start: Vec3<f64>,
    pub dir: Vec3<f64>,
    pub color: Vec3<f64>,
    pub origin: usize,
    pub hops: usize,
}
