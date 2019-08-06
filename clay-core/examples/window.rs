use std::{
    io::Write,
    fs::File,
};
use ocl::{Platform, Device};
use vecmat::{vec::*, mat::*};
use clay_core::{
    Context, Worker,
    scene::ListScene, view::ProjView, map::*,
    shape::*, material::*, object::Covered,
    shape_select, material_combine,
};
use clay_gui::{Window};

shape_select!(MyShape, {
    Cube(Cube),
    Sphere(Sphere),
});
material_combine!(MyMaterial, {
    reflect: Reflective,
    diffuse: Colored<Diffuse>,
});
type MyObject = Covered<Mapper<MyShape, Affine>, MyMaterial>;
type MyScene = ListScene<MyObject>;
type MyView = ProjView;

fn main_() -> Result<(), clay_core::Error> {
    let platform = Platform::default();
    let device = Device::first(platform)?;

    let context = Context::new(platform, device)?;
    let mut worker = Worker::<MyScene, MyView>::new(&context)?;
    File::create("__gen_kernel.c")?.write_all(worker.programs().render.source().as_bytes())?;

    let objects = vec![
        MyShape::Cube(Cube::new())
        .map(Affine::from(
            Mat3::<f64>::from(
                5.0, 0.0, 0.0,
                0.0, 5.0, 0.0,
                0.0, 0.0, 0.1,
            ),
            Vec3::from(0.0, 0.0, -0.1)),
        )
        .cover(MyMaterial::new(
            (Reflective {}, 0.1),
            (Diffuse {}.color_with(Vec3::from(0.9, 0.9, 0.9)), 0.9),
        )),
        
        MyShape::Cube(Cube::new())
        .map(Affine::from(
            0.4*Mat3::<f64>::one(),
            Vec3::from(1.0, 0.0, 0.4)),
        )
        .cover(MyMaterial::new(
            (Reflective {}, 0.1),
            (Diffuse {}.color_with(Vec3::from(0.5, 0.5, 0.9)), 0.9),
        )),
        
        MyShape::Sphere(Sphere::new())
        .map(Affine::from(
            0.5*Mat3::<f64>::one(),
            Vec3::from(0.0, 1.0, 0.5)),
        )
        .cover(MyMaterial::new(
            (Reflective {}, 0.1),
            (Diffuse {}.color_with(Vec3::from(0.9, 0.5, 0.5)), 0.9),
        )),
        
        MyShape::Sphere(Sphere::new())
        .map(Affine::from(
            0.25*Mat3::<f64>::one(),
            Vec3::from(0.0, 0.0, 0.25)),
        )
        .cover(MyMaterial::new(
            (Reflective {}, 0.1),
            (Diffuse {}.color_with(Vec3::from(0.5, 0.9, 0.5)), 0.9),
        )),
        
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
