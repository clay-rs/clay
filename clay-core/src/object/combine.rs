/// Object obtained by covering geometry with material
pub struct Combined<G: Geometry, M: Material> {
    geom: G,
    mat: M,
}

pub trait Combine {
    
}
