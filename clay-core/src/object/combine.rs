/// Object obtained by covering shape with material
pub struct Combined<G: Shape, M: Material> {
    geom: G,
    mat: M,
}

pub trait Combine {
    
}
