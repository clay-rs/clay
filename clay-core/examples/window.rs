use std::{
    env,
    io::Write,
    fs::{File, create_dir_all},
};
use ocl::{Platform, Device};
use nalgebra::{Vector3, Matrix3};
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
    let worker_builder = Worker::<MyScene, MyView>::builder().unwrap();
    
    create_dir_all("./__gen_programs").unwrap();
    for (name, prog) in [
        ("render", &worker_builder.programs().render),
        ("draw", &worker_builder.programs().draw),
    ].iter() {
        File::create(&format!("__gen_programs/{}.c", name)).unwrap()
        .write_all(prog.source().as_bytes()).unwrap();
    }

    let mut worker = worker_builder.build(&context).unwrap();
    for (name, msg) in [
        ("render", &worker.programs().render.1),
        ("draw", &worker.programs().draw.1),
    ].iter() {
        if msg.len() > 0 {
            println!("'{}' build log:\n{}", name, msg);
        }
    }

    let mut builder = ListScene::builder();
    builder.add_targeted(
        MyShape::from(Parallelepiped::build(
            0.25*Matrix3::identity(),
            Vector3::new(-2.0, 0.0, 5.0),
        ))
        .cover(MyMaterial::from(
            Luminous {}.color_with(100.0*Vector3::new(1.0, 1.0, 0.5)),
        ))
    );
    builder.add_targeted(
        MyShape::from(Ellipsoid::build(
            0.2*Matrix3::identity(),
            Vector3::new(0.0, -2.0, 2.5),
        ))
        .cover(MyMaterial::from(
            Luminous {}.color_with(100.0*Vector3::new(0.2, 0.2, 1.0)),
        ))
    );
    builder.add(
        MyShape::from(Parallelepiped::build(
            Matrix3::from_diagonal(&Vector3::new(5.0, 5.0, 0.1)),
            Vector3::new(0.0, 0.0, -0.1),
        ))
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9)),
        ))
    );
    builder.add(
        MyShape::from(Parallelepiped::build(
            0.25*Matrix3::identity(),
            Vector3::new(1.0, 0.0, 0.25),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.2, Reflective {}),
            (0.8, Diffuse {}.color_with(Vector3::new(0.5, 0.5, 0.9))),
        )))
    );
    builder.add(
        MyShape::from(Ellipsoid::build(
            0.25*Matrix3::identity(),
            Vector3::new(0.0, 1.0, 0.25),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.1, Reflective {}),
            (0.9, Diffuse {}.color_with(Vector3::new(0.9, 0.5, 0.5))),
        )))
    );
    builder.add(
        MyShape::from(Ellipsoid::build(
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
