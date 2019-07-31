use vecmat::{vec::*, mat::*};


/// Something that could be packed to `i32` buffers
pub trait PackInt {
    /// Size of integer part of an object.
    fn size_int() -> usize;
    
    /// Write an object into `i32` buffer.
    ///
    /// Buffer *must* be of size greater or equal to object's one.
    fn pack_int_to(&self, buffer: &mut [i32]);
}

/// Something that could be packed to `f32` buffers
pub trait PackFloat {
    /// Size of float part of an object.
    fn size_float() -> usize;

    /// Write an object into `f32` buffer.
    ///
    /// Buffer *must* be of size greater or equal to object's one.
    fn pack_float_to(&self, buffer: &mut [f32]);
}

/// Something that could be packed to a pair of `i32` and `f32` buffers
pub trait Pack {
    /// Size of integer part of an object.
    fn size_int() -> usize;

    /// Size of float part of an object.
    fn size_float() -> usize;

    /// Write an object into int and float buffers.
    ///
    /// Buffers *must* be of size greater or equal to object's one.
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]);
}

pub trait PackerInt {
    fn pack<T: PackInt>(self, t: &T) -> Self;
}
impl<'a> PackerInt for &'a mut [i32] {
    fn pack<T: PackInt>(self, t: &T) -> Self {
        t.pack_int_to(self);
        &mut self[T::size_int()..]
    }
}

pub trait PackerFloat {
    fn pack<T: PackFloat>(self, t: &T) -> Self;
}
impl<'a> PackerFloat for &'a mut [f32] {
    fn pack<T: PackFloat>(self, t: &T) -> Self {
        t.pack_float_to(self);
        &mut self[T::size_float()..]
    }
}

pub struct Packer<'a> {
    buffer_int: &'a mut [i32],
    buffer_float: &'a mut [f32],
}
impl<'a> Packer<'a> {
    pub fn new(buffer_int: &'a mut [i32], buffer_float: &'a mut [f32]) -> Self {
        Self { buffer_int, buffer_float }
    }
    pub fn pack<T: Pack>(self, t: &T) -> Self {
        t.pack_to(self.buffer_int, self.buffer_float);
        Self {
            buffer_int: &mut self.buffer_int[T::size_int()..],
            buffer_float: &mut self.buffer_float[T::size_float()..],
        }
    }
}

impl PackFloat for f32 {
    fn size_float() -> usize { 1 }
    fn pack_float_to(&self, buffer: &mut [f32]) {
        buffer[0] = *self;
    }
}
impl Pack for f32 {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 1 }
    fn pack_to(&self, _buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        buffer_float[0] = *self;
    }
}
impl PackFloat for f64 {
    fn size_float() -> usize { 1 }
    fn pack_float_to(&self, buffer: &mut [f32]) {
        buffer[0] = *self as f32;
    }
}
impl Pack for f64 {
    fn size_int() -> usize { 0 }
    fn size_float() -> usize { 1 }
    fn pack_to(&self, _buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        buffer_float[0] = *self as f32;
    }
}

impl<T: PackFloat + Copy> PackFloat for Vec3<T> {
    fn size_float() -> usize { 3*T::size_float() }
    fn pack_float_to(&self, mut buffer: &mut [f32]) {
        for x in self.iter() {
            buffer = buffer.pack(x);
        }
    }
}
impl<T: Pack + Copy> Pack for Vec3<T> {
    fn size_int() -> usize { 3*T::size_int() }
    fn size_float() -> usize { 3*T::size_float() }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        let mut packer = Packer::new(buffer_int, buffer_float);
        for x in self.iter() {
            packer = packer.pack(x);
        }
    }
}

impl<T: PackFloat + Copy> PackFloat for Mat3<T> {
    fn size_float() -> usize { 9*T::size_float() }
    fn pack_float_to(&self, mut buffer: &mut [f32]) {
        for x in self.iter() {
            buffer = buffer.pack(x);
        }
    }
}
impl<T: Pack + Copy> Pack for Mat3<T> {
    fn size_int() -> usize { 9*T::size_int() }
    fn size_float() -> usize { 9*T::size_float() }
    fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
        let mut packer = Packer::new(buffer_int, buffer_float);
        for x in self.iter() {
            packer = packer.pack(x);
        }
    }
}
