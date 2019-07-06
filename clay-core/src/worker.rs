use std::fs;
use std::path::{Path};

use ocl::{
    self,
    flags,
    Platform,
    Device,
    Context,
    Queue,
    Program,
    Buffer,
    Kernel,
};

#[allow(dead_code)]
pub struct Worker {
    platform:  Platform,
    device:    Device,
    context:   Context,
    program:   Program,
    queue:     Queue,
}

pub struct Screen {
    buffer: Buffer<u8>,
    dims: (usize, usize),
}

impl Worker {
    pub fn new(platform: Platform, device: Device) -> crate::Result<Self> {
        let src_path = Path::new("../clay-core/ocl_src/main.c");
        let src = fs::read_to_string(src_path)?;

        let context = Context::builder()
        .platform(platform)
        .devices(device.clone())
        .build()?;

        let program = Program::builder()
        .devices(device)
        .src(src)
        .build(&context)?;

        let queue = Queue::new(&context, device, None)?;

        Ok(Self { platform, device, context, program, queue })
    }

    pub fn create_screen(&self, size: (usize, usize)) -> crate::Result<Screen> {
        let len = size.0*size.1;

        let mut buffer = Buffer::<u8>::builder()
        .queue(self.queue.clone())
        .flags(flags::MEM_READ_WRITE)
        .len(4*len)
        .fill_val(0 as u8)
        .build()?;

        buffer.set_default_queue(self.queue.clone());

        Ok(Screen { buffer, dims: size })
    }

    pub fn render(&self, screen: &mut Screen) -> crate::Result<()> {
        let size = screen.size();
        let len = size.0*size.1;

        let kernel = Kernel::builder()
        .program(&self.program)
        .name("fill")
        .queue(self.queue.clone())
        .global_work_size(len)
        .arg(&(screen.size().0 as i32))
        .arg(&(screen.size().1 as i32))
        .arg(&screen.buffer)
        .build()?;

        unsafe {
            kernel.cmd()
            .queue(&self.queue)
            .global_work_offset(kernel.default_global_work_offset())
            .global_work_size(len)
            .local_work_size(kernel.default_local_work_size())
            .enq()?;
        }

        Ok(())
    }
}

impl Screen {
    pub fn read(&self) -> crate::Result<Vec<u8>> {
        let mut vec = vec![0 as u8; self.buffer.len()];

        self.buffer.cmd()
        .offset(0)
        .read(&mut vec)
        .enq()?;

        Ok(vec)
    }
    
    pub fn size(&self) -> (usize, usize) {
        self.dims
    }
}
