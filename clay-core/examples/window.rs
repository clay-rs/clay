use ocl::{Platform, Device};

use clay_core::{Context, Worker};
use clay_gui::{Window};


fn main() -> Result<(), clay_core::Error> {
    let platform = Platform::default();
    let device = Device::first(platform)?;

    let context = Context::new(platform, device)?;
    let worker = Worker::new(&context)?;

    let mut window = Window::new((800, 600))?;

    window.start(&context, &worker)?;

    Ok(())
}
