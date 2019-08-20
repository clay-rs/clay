use std::{
    env,
    io::Write,
    fs::{File, create_dir_all},
};
use ocl::{Platform, Device};
use nalgebra::{Vector3, Matrix3};
use clay_core::{
    Context, Worker,
    shape::*, material::*, object::Covered,
    shape_select, material_select, material_combine,
};
use clay::{
    scene::TargetListScene, view::ProjView,
    shape::*, material::*,
    background::{GradientBackground as GradBg},
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
type MyScene = TargetListScene<MyObject, Sphere, GradBg>;
type MyView = ProjView;


fn main() {
    let args = env::args().collect::<Vec<_>>();
    let platform = if args.len() > 1 {
        let platform_list = Platform::list();
        let index = args[1].parse::<usize>().unwrap();
        assert!(platform_list.len() > index);
        platform_list[index]
    } else {
        Platform::default()
    };
    let device = Device::first(platform).unwrap();

    let context = Context::new(platform, device).unwrap();
    let mut builder = Worker::<MyScene, MyView>::builder();
    builder.add_hook(clay_core::source());
    builder.add_hook(clay::source());
    let builder = builder.collect().unwrap();

    create_dir_all("./__gen_programs").unwrap();
    for (name, prog) in [
        ("render", &builder.programs().render),
        ("draw", &builder.programs().draw),
    ].iter() {
        File::create(&format!("__gen_programs/{}.c", name)).unwrap()
        .write_all(prog.source().as_bytes()).unwrap();
    }

    let mut worker = builder.build(&context).unwrap();
    for (name, msg) in [
        ("render", &worker.programs().render.1),
        ("draw", &worker.programs().draw.1),
    ].iter() {
        if msg.len() > 0 {
            println!("'{}' build log:\n{}", name, msg);
        }
    }

    let mut builder = TargetListScene::builder(GradBg::new(
        Vector3::new(0.2, 0.2, 0.4), Vector3::zeros(),
    ));
    builder.add_targeted(
        MyShape::from(Parallelepiped::new(
            0.25*Matrix3::identity(),
            Vector3::new(-2.0, 0.0, 5.0),
        ))
        .cover(MyMaterial::from(
            Luminous {}.color_with(100.0*Vector3::new(1.0, 1.0, 0.5)),
        ))
    );
    builder.add_targeted(
        MyShape::from(Ellipsoid::new(
            0.2*Matrix3::identity(),
            Vector3::new(0.0, -2.0, 2.5),
        ))
        .cover(MyMaterial::from(
            Luminous {}.color_with(100.0*Vector3::new(0.2, 0.2, 1.0)),
        ))
    );
    builder.add(
        MyShape::from(Parallelepiped::new(
            Matrix3::from_diagonal(&Vector3::new(5.0, 5.0, 0.1)),
            Vector3::new(0.0, 0.0, -0.1),
        ))
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9)),
        ))
    );
    builder.add(
        MyShape::from(Parallelepiped::new(
            0.25*Matrix3::identity(),
            Vector3::new(1.0, 0.0, 0.25),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.2, Reflective {}),
            (0.8, Diffuse {}.color_with(Vector3::new(0.5, 0.5, 0.9))),
        )))
    );
    builder.add(
        MyShape::from(Ellipsoid::new(
            0.25*Matrix3::identity(),
            Vector3::new(0.0, 1.0, 0.25),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.1, Reflective {}),
            (0.9, Diffuse {}.color_with(Vector3::new(0.9, 0.5, 0.5))),
        )))
    );
    builder.add(
        MyShape::from(Ellipsoid::new(
            0.5*Matrix3::identity(),
            Vector3::new(0.0, 0.0, 0.5),
        ))
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vector3::new(0.5, 0.9, 0.5)),
        ))
    );
    let scene = builder.build(&context).unwrap();


    let mut window = Window::new((1000, 800)).unwrap();

    window.start(&context, |screen, pos, map| {
        let view = ProjView {
            pos: pos + Vector3::new(0.0, -2.0, 1.0),
            ori: map.matrix().clone(),
        };
        worker.render(screen, &scene, &view)
    }).unwrap();
}
