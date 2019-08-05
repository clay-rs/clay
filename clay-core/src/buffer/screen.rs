use ocl;
use rand::{Rng, thread_rng};
use crate::Context;


pub struct Screen {
    random: ocl::Buffer<u32>,
    color: ocl::Buffer<f32>,
    passes: usize,
    bytes: ocl::Buffer<u8>,
    dims: (usize, usize),
}

impl Screen {
    pub fn new(context: &Context, dims: (usize, usize)) -> crate::Result<Screen> {
        let len = dims.0*dims.1;

        let random = ocl::Buffer::<u32>::builder()
        .queue(context.queue().clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len(len)
        .fill_val(0 as u32)
        .build()?;
        let mut seed = vec![0u32; len];
        thread_rng().fill(&mut seed[..]);
        random.cmd()
        .offset(0)
        .write(&seed)
        .enq()?;

        let color = ocl::Buffer::<f32>::builder()
        .queue(context.queue().clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len(3*len)
        .fill_val(0 as f32)
        .build()?;

        let bytes = ocl::Buffer::<u8>::builder()
        .queue(context.queue().clone())
        .flags(ocl::flags::MEM_WRITE_ONLY)
        .len(3*len)
        .fill_val(0 as u8)
        .build()?;

        Ok(Screen {
            random,
            color, passes: 0,
            bytes, dims,
        })
    }
    
    pub fn read(&self) -> crate::Result<Vec<u8>> {
        let mut vec = vec![0 as u8; self.bytes.len()];

        self.bytes.cmd()
        .offset(0)
        .read(&mut vec)
        .enq()?;

        Ok(vec)
    }
    
    pub fn random(&self) -> &ocl::Buffer<u32> {
        &self.random
    }
    pub fn random_mut(&mut self) -> &mut ocl::Buffer<u32> {
        &mut self.random
    }
    pub fn color(&self) -> &ocl::Buffer<f32> {
        &self.color
    }
    pub fn color_mut(&mut self) -> &mut ocl::Buffer<f32> {
        &mut self.color
    }
    pub fn passes(&self) -> usize {
        self.passes
    }
    pub fn bytes(&self) -> &ocl::Buffer<u8> {
        &self.bytes
    }
    pub fn bytes_mut(&mut self) -> &mut ocl::Buffer<u8> {
        &mut self.bytes
    }

    pub fn dims(&self) -> (usize, usize) {
        self.dims
    }
    pub fn len(&self) -> usize {
        self.dims.0*self.dims.1
    }
}
