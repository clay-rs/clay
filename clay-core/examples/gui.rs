use ocl::{Platform, Device};

use clay_core::{Worker};
use clay_gui::{Window};


fn main() {
    let platform = Platform::default();
    let device = Device::first(platform).unwrap();

    let mut worker = Worker::new(platform, device).unwrap();

    let mut window = Window::new((800, 600)).unwrap();

    window.start(&mut worker).unwrap();
}
