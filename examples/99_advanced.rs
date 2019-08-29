use std::{
    env,
    io::Write,
    fs::{File, create_dir_all},
};
use ocl::{Platform, Device};
use nalgebra::{Vector3, Matrix3};
use clay_core::{
    Context, Store,
    shape::*, material::*, object::Covered,
    shape_select, material_select, material_combine,
};
use clay::{
    scene::TargetListScene, view::ProjView,
    shape::*, material::*,
    background::{GradientBackground as GradBg},
    filter::{GlareFilter},
    process::{DefaultRenderer, Postproc},
};
use clay_viewer::{Window};

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
    // Parse args to select OpenCL platform
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
    let dims = (1000, 800);

    let mut scene = TargetListScene::new(GradBg::new(
        Vector3::new(0.2, 0.2, 0.4), Vector3::zeros(),
        Vector3::new(0.0, 0.0, 1.0),
    ));
    scene.add_targeted(
        MyShape::from(Parallelepiped::new(
            0.25*Matrix3::identity(),
            Vector3::new(-2.0, 0.0, 5.0),
        ))
        .cover(MyMaterial::from(
            Luminous {}.color_with(100.0*Vector3::new(1.0, 1.0, 0.5)),
        ))
    );
    scene.add_targeted(
        MyShape::from(Ellipsoid::new(
            0.2*Matrix3::identity(),
            Vector3::new(0.0, -2.0, 2.5),
        ))
        .cover(MyMaterial::from(
            Luminous {}.color_with(100.0*Vector3::new(0.2, 0.2, 1.0)),
        ))
    );
    scene.add(
        MyShape::from(Parallelepiped::new(
            Matrix3::from_diagonal(&Vector3::new(5.0, 5.0, 0.1)),
            Vector3::new(0.0, 0.0, -0.1),
        ))
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9)),
        ))
    );
    scene.add(
        MyShape::from(Parallelepiped::new(
            0.25*Matrix3::identity(),
            Vector3::new(1.0, 0.0, 0.25),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.2, Reflective {}),
            (0.8, Diffuse {}.color_with(Vector3::new(0.5, 0.5, 0.9))),
        )))
    );
    scene.add(
        MyShape::from(Ellipsoid::new(
            0.25*Matrix3::identity(),
            Vector3::new(0.0, 0.0, 0.25),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.1, Reflective {}),
            (0.9, Diffuse {}.color_with(Vector3::new(0.9, 0.5, 0.5))),
        )))
    );
    scene.add(
        MyShape::from(Ellipsoid::new(
            0.5*Matrix3::identity(),
            Vector3::new(0.5, 1.0, 0.5),
        ))
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vector3::new(0.5, 0.9, 0.5)),
        ))
    );

    let origin = Vector3::new(0.0, -2.0, 1.0);
    let view = ProjView {
        pos: origin,
        ori: Matrix3::identity(),
    };

    let filter = GlareFilter::new(0.01);

    let mut renderer = DefaultRenderer::<MyScene, MyView>::new(dims, scene, view).unwrap();
    let postproc_builder = Postproc::<GlareFilter>::builder().unwrap();

    create_dir_all("./__gen_programs").unwrap();
    for (name, prog) in [
        ("render.c", &renderer.program()),
        ("filter.c", &postproc_builder.program()),
    ].iter() {
        File::create(&format!("__gen_programs/{}", name)).unwrap()
        .write_all(prog.source().as_bytes()).unwrap();
    }

    let (mut worker, message) = renderer.create_worker(&context).unwrap();
    if message.len() > 0 {
        println!("render build log:\n{}", message);
    }

    let (mut postproc, message) = postproc_builder.build(&context, dims, filter).unwrap();
    if message.len() > 0 {
        println!("filter build log:\n{}", message);
    }

    let mut window = Window::new(dims).unwrap();

    'main: loop {
        let quit = window.poll().unwrap();
        if quit {
            break 'main;
        }

        let (pos, ori) = window.view_params();
        renderer.view.pos = pos + origin;
        renderer.view.ori = ori.matrix().clone();
        renderer.update_data(&context, worker.data_mut()).unwrap();

        if window.was_updated() {
            worker.data_mut().buffer_mut().clear().unwrap();
        }
        worker.run().unwrap();

        postproc.process_one(&worker.data().buffer()).unwrap();
        postproc.make_image().unwrap();

        window.draw(postproc.image()).unwrap();
    }
}
