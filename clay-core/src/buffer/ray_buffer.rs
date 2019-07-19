use ocl;

use crate::Context;


pub struct RayBuffer {
    buffer_int: ocl::Buffer<u32>,
    buffer_float: ocl::Buffer<f32>,
    size_int: usize,
    size_float: usize,
    count: usize,
}

impl ObjectBuffer {
    pub fn new(
        context: &Context,
        size_int: usize, size_float: usize,
        count: usize
    ) -> crate::Result<ObjectBuffer> {
        let buffer_int = ocl::Buffer::<u32>::builder()
        .queue(context.queue().clone())
        .flags(ocl::flags::MEM_READ_ONLY)
        .len(size_int*count)
        .fill_val(0 as u32)
        .build()?;

        let buffer_float = ocl::Buffer::<f32>::builder()
        .queue(context.queue().clone())
        .flags(ocl::flags::MEM_READ_ONLY)
        .len(size_int*count)
        .fill_val(0 as f32)
        .build()?;

        Ok(ObjectBuffer {
            buffer_int, buffer_float,
            size_int, size_float,
            count,
        })
    }
    
    pub fn write(&mut self, buffer_int: &[u32], buffer_float: &[f32]) -> crate::Result<()> {
        if buffer_int.len() == self.buffer_int.len() && buffer_float.len() == self.buffer_float.len() {
            self.buffer_int.cmd()
            .offset(0)
            .write(buffer_int)
            .enq()?;

            self.buffer_float.cmd()
            .offset(0)
            .write(buffer_float)
            .enq()?;

            Ok(())
        } else {
            Err("buffers size mismatch".into())
        }
    }
    
    pub fn buffer_int(&self) -> &ocl::Buffer<u32> {
        &self.buffer_int
    }
    pub fn buffer_float(&self) -> &ocl::Buffer<f32> {
        &self.buffer_float
    }

    pub fn size_int(&self) -> usize {
        self.size_int
    }
    pub fn size_float(&self) -> usize {
        self.size_float
    }
    pub fn count(&self) -> usize {
        self.count
    }
}
