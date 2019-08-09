use std::{
    io::Write,
    fs::File,
};
use ocl::{Platform, Device};
use vecmat::{vec::*, mat::*};
use clay_core::{
    Context, Worker,
    scene::ListScene, view::ProjView, map::*,
    shape::*, material::*, object::Covered, attract::*,
    shape_select, material_select, material_combine,
};
use clay_gui::{Window};

shape_select!(MyShape, {
    Cube(Cube),
    Sphere(Sphere),
});
material_combine!(Glossy, {
    reflect: Reflective,
    diffuse: Colored<Diffuse>,
});
material_select!( MyMaterial, {
    Matte(Colored<Diffuse>),
    Glossy(Glossy),
    Luminous(Colored<Luminous>),
});
type MyObject = Covered<Mapper<MyShape, Affine>, MyMaterial>;
type MyScene = ListScene<MyObject, SphereAttractor>;
type MyView = ProjView;

fn main() {
    let platform = Platform::default();
    let device = Device::first(platform).unwrap();

    let context = Context::new(platform, device).unwrap();
    let mut worker = Worker::<MyScene, MyView>::new(&context).unwrap();
    File::create("__gen_kernel.c").unwrap().write_all(worker.programs().render.source().as_bytes()).unwrap();

    let omni_size = 0.25;
    let omni_pos = Vec3::from(0.0, 0.0, 3.5);
    let objects = vec![
        MyShape::from(Cube::new())
        .map(
            Linear::from(omni_size*Mat3::<f64>::one())
            .chain(Shift::from(omni_pos))
        )
        .cover(MyMaterial::from(
            Luminous {}.color_with(100.0*Vec3::from(1.0, 1.0, 1.0)),
        )),

        MyShape::from(Cube::new())
        .map(
            Linear::from(Mat3::<f64>::from(
                5.0, 0.0, 0.0,
                0.0, 5.0, 0.0,
                0.0, 0.0, 0.1,
            ))
            .chain(Shift::from(Vec3::from(0.0, 0.0, -0.1)))
        )
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vec3::from(0.9, 0.9, 0.9)),
        )),
        
        MyShape::from(Cube::new())
        .map(
            Linear::from(0.4*Mat3::<f64>::one())
            .chain(Shift::from(Vec3::from(1.0, 0.0, 0.4)))
        )
        .cover(MyMaterial::from(Glossy::new(
            (0.1, Reflective {}),
            (0.9, Diffuse {}.color_with(Vec3::from(0.5, 0.5, 0.9))),
        ))),
        
        MyShape::from(Sphere::new())
        .map(
            Linear::from(0.5*Mat3::<f64>::one())
            .chain(Shift::from(Vec3::from(0.0, 1.0, 0.5)))
        )
        .cover(MyMaterial::from(Glossy::new(
            (0.1, Reflective {}),
            (0.9, Diffuse {}.color_with(Vec3::from(0.9, 0.5, 0.5))),
        ))),
        
        MyShape::from(Sphere::new())
        .map(
            Linear::from(0.25*Mat3::<f64>::one())
            .chain(Shift::from(Vec3::from(0.0, 0.0, 0.25)))
        )
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vec3::from(0.5, 0.9, 0.5)),
        )),
    ];
    let attractors = vec![
        SphereAttractor::new(Sphere::new().map(
            Scale::from(omni_size*f64::sqrt(2.0))
            .chain(Shift::from(omni_pos))
        ), 0),
    ];
    let scene = MyScene::new(&context, objects, attractors).unwrap();

    let mut window = Window::new((1000, 800)).unwrap();

    window.start(&context, |screen, pos, map| {
        let view = ProjView {
            pos: pos + Vec3::from(0.0, -1.0, 0.5),
            ori: map,
        };
        worker.render(screen, &scene, &view)
    }).unwrap();
}
