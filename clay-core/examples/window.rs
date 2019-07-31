use ocl::{Platform, Device};
use vecmat::{vec::*, mat::*};
use clay_core::{
    Context, Worker,
    scene::ListScene, view::ProjView, map::*,
    shape::*, material::Mirror, object::Covered,
};
use clay_gui::{Window};


type MirrorSphere = Covered<Mapper<Sphere, Affine>, Mirror>;
type MyScene = ListScene<MirrorSphere>;
type MyView = ProjView;

fn main() -> Result<(), clay_core::Error> {
    let platform = Platform::default();
    let device = Device::first(platform)?;

    let context = Context::new(platform, device)?;
    let mut worker = Worker::<MyScene, MyView>::new(&context)?;

    let mut ma = Mat3::<f64>::one();
    ma[(2, 2)] = 0.75;
    let mut mb = 0.75*Mat3::<f64>::one();
    mb[(2, 2)] = 1.0;
    let objects = vec![
        MirrorSphere::new(
            Sphere::new().map(Affine::from(ma, Vec3::from(0.0, 5.0, 0.0))),
            Mirror { color: Vec3::from(0.7, 0.7, 0.9) },
        ),
        MirrorSphere::new(
            Sphere::new().map(Affine::from(mb, Vec3::from(2.0, 3.0, 0.0))),
            Mirror { color: Vec3::from(0.9, 0.7, 0.7) },
        ),
    ];
    let scene = MyScene::new(objects, &context)?;

    let mut window = Window::new((1000, 800))?;

    window.start(&context, |screen, pos, map| {
        let view = ProjView { pos, ori: map };
        worker.render(screen, &scene, &view)
    })?;

    Ok(())
}
