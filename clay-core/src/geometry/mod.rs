mod sphere;
pub use sphere::Sphere;

use crate::Pack;


/// Abstract object
pub trait Geometry: Pack {
    /// Name of function from OpenCL source that could be used to
    /// find a ray intersection with this geometry.
    fn ocl_hit_fn() -> &'static str;

    /// Returns bounding sphere - the sphere that contains the whole object inside.
    ///
    /// If the object is borderless and doesn't fit into any sphere
    /// then `None` should be returned.
    fn bounds(&self) -> Option<Sphere>;
}
