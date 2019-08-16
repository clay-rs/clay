use std::{
    io::Write,
    fs::File,
};
use ocl::{Platform, Device};
use vecmat::{vec::*, mat::*};
use clay_core::{
    Context, Worker,
    scene::ListScene, view::ProjView,
    shape::*, material::*, object::Covered,
    shape_select, material_select, material_combine,
};
use clay_gui::{Window};

shape_select!(MyShape {
    Cube(TC=Parallelepiped),
    Sphere(TS=Ellipsoid),
});
material_combine!(Glossy {
    reflect: Reflective,
    diffuse: Colored<Diffuse>,
});
material_select!(MyMaterial {
    Matte(TM=Colored<Diffuse>),
    Glossy(TG=Glossy),
    Luminous(TC=Colored<Luminous>),
});
type MyObject = Covered<MyShape, MyMaterial>;
type MyScene = ListScene<MyObject, Sphere>;
type MyView = ProjView;

fn main() {
    let platform = Platform::default();
    let device = Device::first(platform).unwrap();

    let context = Context::new(platform, device).unwrap();
    let mut worker = Worker::<MyScene, MyView>::new(&context).unwrap();
    File::create("__gen_kernel.c").unwrap().write_all(worker.programs().render.source().as_bytes()).unwrap();

    let mut builder = ListScene::builder();
    builder.add_targeted(
        MyShape::from(Parallelepiped::build(
            0.25*Mat3::<f64>::one(),
            Vec3::from(-2.0, 0.0, 5.0),
        ))
        .cover(MyMaterial::from(
            Luminous {}.color_with(100.0*Vec3::from(1.0, 1.0, 0.5)),
        ))
    );
    builder.add_targeted(
        MyShape::from(Ellipsoid::build(
            0.2*Mat3::<f64>::one(),
            Vec3::from(0.0, -2.0, 2.5),
        ))
        .cover(MyMaterial::from(
            Luminous {}.color_with(100.0*Vec3::from(0.2, 0.2, 1.0)),
        ))
    );
    builder.add(
        MyShape::from(Parallelepiped::build(
            Mat3::<f64>::from(
                5.0, 0.0, 0.0,
                0.0, 5.0, 0.0,
                0.0, 0.0, 0.1,
            ),
            Vec3::from(0.0, 0.0, -0.1),
        ))
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vec3::from(0.9, 0.9, 0.9)),
        ))
    );
    builder.add(
        MyShape::from(Parallelepiped::build(
            0.25*Mat3::<f64>::one(),
            Vec3::from(1.0, 0.0, 0.25),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.2, Reflective {}),
            (0.8, Diffuse {}.color_with(Vec3::from(0.5, 0.5, 0.9))),
        )))
    );
    builder.add(
        MyShape::from(Ellipsoid::build(
            0.25*Mat3::<f64>::one(),
            Vec3::from(0.0, 1.0, 0.25),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.1, Reflective {}),
            (0.9, Diffuse {}.color_with(Vec3::from(0.9, 0.5, 0.5))),
        )))
    );
    builder.add(
        MyShape::from(Ellipsoid::build(
            0.5*Mat3::<f64>::one(),
            Vec3::from(0.0, 0.0, 0.5),
        ))
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vec3::from(0.5, 0.9, 0.5)),
        ))
    );
    let scene = builder.build(&context).unwrap();


    let mut window = Window::new((1000, 800)).unwrap();

    window.start(&context, |screen, pos, map| {
        let view = ProjView {
            pos: pos + Vec3::from(0.0, -1.0, 0.5),
            ori: map,
        };
        worker.render(screen, &scene, &view)
    }).unwrap();
}
