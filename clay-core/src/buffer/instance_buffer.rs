use std::{
    marker::PhantomData,
};
use ocl::{self, builders::KernelBuilder};
use crate::{
    Context,
    Pack, Push,
};


pub struct InstanceBuffer<T: Pack> {
    buffer_int: ocl::Buffer<i32>,
    buffer_float: ocl::Buffer<f32>,
    count: usize,
    phantom: PhantomData<T>,
}

impl<T: Pack> InstanceBuffer<T> {
    pub fn new(context: &Context, objects: &[T]) -> crate::Result<Self> {
        let mut buffer = Self::reserved(context, objects.len())?;
        buffer.write(&objects)?;
        Ok(buffer)
    }

    pub fn reserved(context: &Context, count: usize) -> crate::Result<Self> {
        let buffer_int = ocl::Buffer::<i32>::builder()
        .queue(context.queue().clone())
        .flags(ocl::flags::MEM_READ_ONLY)
        .len((T::size_int()*count).max(1))
        .fill_val(0 as i32)
        .build()?;

        let buffer_float = ocl::Buffer::<f32>::builder()
        .queue(context.queue().clone())
        .flags(ocl::flags::MEM_READ_ONLY)
        .len((T::size_float()*count).max(1))
        .fill_val(0 as f32)
        .build()?;

        Ok(Self {
            buffer_int, buffer_float,
            count, phantom: PhantomData::<T>,
        })
    }
    /*
    pub fn write_iter<'b, I: Iterator<Item=&'b T>>(&mut self, objects: I) -> crate::Result<()> {
        let objvec = objects.collect::<Vec<_>>();
        self.write_slice(&objvec)
    }
    */

    pub fn write(&mut self, objects: &[T]) -> crate::Result<()> {
        let mut buffer_int = vec![0i32; T::size_int().max(1)*objects.len()];
        let mut buffer_float = vec![0.0f32; T::size_float().max(1)*objects.len()];
        // Use this `.max(1)` workaround because `chunks` panics on 0 (why there is such silly requirement?)
        for (obj, (ibuf, fbuf)) in objects.iter().zip(
            buffer_int.chunks_mut(Self::size_int().max(1))
            .zip(buffer_float.chunks_mut(Self::size_float().max(1)))
        ) {
            obj.pack_to(&mut ibuf[..T::size_int()], &mut fbuf[..T::size_float()]);
        }
        if objects.len() == 0 || T::size_int() == 0 { buffer_int = vec![0]; }
        if objects.len() == 0 || T::size_float() == 0 { buffer_float = vec![0.0]; }

        if buffer_int.len() == self.buffer_int.len() && buffer_float.len() == self.buffer_float.len() {
            self.buffer_int.cmd()
            .offset(0)
            .write(&buffer_int)
            .enq()?;

            self.buffer_float.cmd()
            .offset(0)
            .write(&buffer_float)
            .enq()?;

            Ok(())
        } else {
            Err("buffers size mismatch".into())
        }
    }
    
    pub fn buffer_int(&self) -> &ocl::Buffer<i32> {
        &self.buffer_int
    }
    pub fn buffer_float(&self) -> &ocl::Buffer<f32> {
        &self.buffer_float
    }

    pub fn size_int() -> usize {
        T::size_int()
    }
    pub fn size_float() -> usize {
        T::size_float()
    }
    pub fn count(&self) -> usize {
        self.count
    }
}


impl<T: Pack> Push for InstanceBuffer<T> {
    fn args_def(kb: &mut KernelBuilder) {
        kb
        .arg(None::<&ocl::Buffer<i32>>) // int buffer
        .arg(None::<&ocl::Buffer<f32>>) // float buffer
        .arg(0i32) // int size
        .arg(0i32) // float size
        .arg(0i32); // instance count
    }
    fn args_set(&self, i: usize, k: &mut ocl::Kernel) -> crate::Result<()> {
        k.set_arg(i + 0, self.buffer_int())?;
        k.set_arg(i + 1, self.buffer_float())?;
        k.set_arg(i + 2, Self::size_int() as i32)?;
        k.set_arg(i + 3, Self::size_float() as i32)?;
        k.set_arg(i + 4, self.count() as i32)?;
        Ok(())
    }
    fn args_count() -> usize {
        5
    }
}
