use std::{
    io::Write,
    fs::File,
};
use ocl::{Platform, Device};
use vecmat::{vec::*, mat::*};
use clay_core::{
    Context, Worker,
    scene::ListScene, view::ProjView, map::*,
    shape::*,
    material::*, object::Covered,
};
use clay_gui::{Window};


type MyShape = Covered<Mapper<Cube, Affine>, Colored<Diffuse>>;
type MyScene = ListScene<MyShape>;
type MyView = ProjView;

fn main_() -> Result<(), clay_core::Error> {
    let platform = Platform::default();
    let device = Device::first(platform)?;

    let context = Context::new(platform, device)?;
    let mut worker = Worker::<MyScene, MyView>::new(&context)?;
    File::create("__gen_kernel.c")?.write_all(worker.programs().render.source().as_bytes())?;

    let objects = vec![
        Cube::new()
        .map(Affine::from(
            Mat3::<f64>::from(
                5.0, 0.0, 0.0,
                0.0, 5.0, 0.0,
                0.0, 0.0, 0.1,
            ),
            Vec3::from(0.0, 0.0, -0.1)),
        )
        .cover(Diffuse {}.color_with(Vec3::from(0.9, 0.9, 0.9))),
        
        Cube::new()
        .map(Affine::from(
            0.5*Mat3::<f64>::one(),
            Vec3::from(1.0, 0.0, 0.5)),
        )
        .cover(Diffuse {}.color_with(Vec3::from(0.5, 0.5, 0.9))),
        
        Cube::new()
        .map(Affine::from(
            0.5*Mat3::<f64>::one(),
            Vec3::from(0.0, 1.0, 0.5)),
        )
        .cover(Diffuse {}.color_with(Vec3::from(0.9, 0.5, 0.5))),
        
        Cube::new()
        .map(Affine::from(
            0.25*Mat3::<f64>::one(),
            Vec3::from(0.0, 0.0, 0.25)),
        )
        .cover(Diffuse {}.color_with(Vec3::from(0.5, 0.9, 0.5))),
        
    ];
    let scene = MyScene::new(objects, &context)?;

    let mut window = Window::new((1000, 800))?;

    window.start(&context, |screen, pos, map| {
        let view = ProjView {
            pos: pos + Vec3::from(0.0, -1.0, 0.5),
            ori: map,
        };
        worker.render(screen, &scene, &view)
    })?;

    Ok(())
}

fn main() {
    match main_() {
        Ok(()) => (),
        Err(err) => panic!("{}", err),
    } 
}
