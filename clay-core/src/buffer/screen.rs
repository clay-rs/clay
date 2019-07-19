use ocl;

use crate::Context;


pub struct Screen {
    //color: ocl::Buffer<f32>,
    //passes: usize,
    bytes: ocl::Buffer<u8>,
    dims: (usize, usize),
}

impl Screen {
    pub fn new(context: &Context, dims: (usize, usize)) -> crate::Result<Screen> {
        let len = dims.0*dims.1;

        let bytes = ocl::Buffer::<u8>::builder()
        .queue(context.queue().clone())
        .flags(ocl::flags::MEM_WRITE_ONLY)
        .len(4*len)
        .fill_val(0 as u8)
        .build()?;

        Ok(Screen { bytes, dims })
    }
    
    pub fn read(&self) -> crate::Result<Vec<u8>> {
        let mut vec = vec![0 as u8; self.bytes.len()];

        self.bytes.cmd()
        .offset(0)
        .read(&mut vec)
        .enq()?;

        Ok(vec)
    }
    
    pub fn buffer(&self) -> &ocl::Buffer<u8> {
        &self.bytes
    }
    pub fn buffer_mut(&mut self) -> &mut ocl::Buffer<u8> {
        &mut self.bytes
    }

    pub fn dims(&self) -> (usize, usize) {
        self.dims
    }
    pub fn len(&self) -> usize {
        self.dims.0*self.dims.1
    }
}
