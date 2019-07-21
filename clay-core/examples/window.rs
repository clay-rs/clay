use ocl::{Platform, Device};
use vecmat::vec::*;
use clay_core::{Context, Worker, buffer::ObjectBuffer, shape::Sphere};
use clay_gui::{Window};


fn main() -> Result<(), clay_core::Error> {
    let platform = Platform::default();
    let device = Device::first(platform)?;

    let context = Context::new(platform, device)?;
    let worker = Worker::new(&context)?;

    let objects = ObjectBuffer::<Sphere>::new(&context, &[
        Sphere { pos: Vec3::from(0.0, 5.0, 0.0), rad: 1.0 },
        Sphere { pos: Vec3::from(2.0, 3.0, 0.0), rad: 0.5 },
    ])?;

    let mut window = Window::new((800, 600))?;

    window.start(&context, |screen, pos, map| {
        worker.render(screen, pos, map, &objects)
    })?;

    Ok(())
}
