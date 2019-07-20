/// Object obtained by covering geometry with material
pub struct Cover<G: Geometry, M: Material> {
    geom: G,
    mat: M,
}
