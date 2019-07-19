/// Something that could be packed to buffers
pub trait Pack {
    /// Size of integer part of an object.
    fn size_int() -> usize;
    /// Size of float part of an object.
    fn size_float() -> usize;

    /// Write an object into int and float buffers.
    ///
    /// Buffers *must* be of size greater or equal to object's one.
    fn pack(&self, buffer_int: &mut [u32], buffer_float: &mut [f32]);

    /// Read an object from int and float buffers.
    ///
    /// Buffers *must* be of size greater or equal to object's one.
    fn unpack(buffer_int: &[u32], buffer_float: &[f32]) -> Self;
}
