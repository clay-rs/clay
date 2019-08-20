use std::env;
use ocl::{Platform, Device};
use nalgebra::{Vector3};
use clay_core::{
    Context,
    shape::*, material::*, object::Covered,
};
use clay::{
    scene::ListScene, view::ProjView,
    shape::*, material::*,
    worker::DefaultWorker,
    background::{GradientBackground as GradBg},
};
use clay_gui::{Window};

// Here we declare our object - a combination of
// spherical shape and colored diffuse material
type MyObject = Covered<Sphere, Colored<Diffuse>>;

type MyScene = ListScene<MyObject, GradBg>;
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
    let worker = DefaultWorker::<MyScene, MyView>::builder().unwrap();

    // Print build log
    let mut worker = worker.build(&context).unwrap();
    for (name, msg) in [
        ("render", &worker.programs().render.1),
        ("draw", &worker.programs().draw.1),
    ].iter() {
        if msg.len() > 0 {
            println!("'{}' build log:\n{}", name, msg);
        }
    }

    let mut scene = ListScene::builder(GradBg::new(
        Vector3::new(0.8, 0.8, 0.8), Vector3::new(0.2, 0.2, 0.2),
    ));
    scene.add(
        Sphere::new(0.75, Vector3::new(-0.75, 0.0, 0.0))
        .cover(Diffuse {}.color_with(Vector3::new(0.3, 0.9, 0.3)))
    );
    scene.add(
        Sphere::new(1.0, Vector3::new(1.0, 0.0, 0.0))
        .cover(Diffuse {}.color_with(Vector3::new(0.3, 0.3, 0.9)))
    );
    let scene = scene.build(&context).unwrap();


    let mut window = Window::new((1000, 800)).unwrap();

    window.start(&context, |screen, pos, map| {
        let view = ProjView {
            pos: pos + Vector3::new(0.0, -2.0, 0.0),
            ori: map.matrix().clone(),
        };
        worker.render(screen, &scene, &view)
    }).unwrap();
}
