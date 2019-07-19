mod sphere;

pub use sphere::Sphere;

use crate::Pack;


/// Abstract object
pub trait Object: Pack {
    /// Returns bounding sphere - the sphere that contains the whole object inside.
    ///
    /// If the object is borderless and doesn't fit into any sphere
    /// then `None` should be returned.
    fn bounds(&self) -> Option<Sphere>;
}
