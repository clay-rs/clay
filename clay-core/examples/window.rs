use ocl::{Platform, Device};
use vecmat::vec::*;
use clay_core::{
    Context, Worker,
    scene::ListScene, view::ProjView,
    shape::Sphere, material::Mirror, object::Covered,
};
use clay_gui::{Window};


type MirrorSphere = Covered<Sphere, Mirror>;
type MyScene = ListScene<MirrorSphere>;
type MyView = ProjView;

fn main() -> Result<(), clay_core::Error> {
    let platform = Platform::default();
    let device = Device::first(platform)?;

    let context = Context::new(platform, device)?;
    let mut worker = Worker::<MyScene, MyView>::new(&context)?;

    let objects = vec![
        MirrorSphere::new(
            Sphere { pos: Vec3::from(0.0, 5.0, 0.0), rad: 1.0 },
            Mirror { color: Vec3::from(0.7, 0.7, 0.9) },
        ),
        MirrorSphere::new(
            Sphere { pos: Vec3::from(2.0, 3.0, 0.0), rad: 0.5 },
            Mirror { color: Vec3::from(0.9, 0.7, 0.7) },
        ),
    ];
    let scene = MyScene::new(objects, &context)?;

    let mut window = Window::new((800, 600))?;

    window.start(&context, |screen, pos, map| {
        let view = ProjView { pos, ori: map };
        worker.render(screen, &scene, &view)
    })?;

    Ok(())
}
