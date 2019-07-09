use super::Sphere;


/// Abstract object
pub trait Object {
    /// Size of integer part of an object.
    fn size_int() -> usize;
    /// Size of float part of an object.
    fn size_float() -> usize;

    /// Name of OpenCL function in source that will be used to trace an object.
    fn ocl_trace_fn() -> String;

    /// Bounding sphere - the sphere that contains the whole object inside.
    ///
    /// If the object is borderless and doesn't fit into any sphere
    /// then `None` should be returned.
    fn bounding_sphere(&self) -> Option<Sphere>;

    /// Write an object into int and float buffers.
    ///
    /// Buffers *must* be of size greater or equal to object's one.
    fn pack(&self, buffer_int: &mut [u32], buffer_float: &mut [f32]);
}
