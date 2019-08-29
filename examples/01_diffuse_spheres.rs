use std::env;
use ocl::{Platform, Device};
use nalgebra::{Vector3, Matrix3};
use clay_core::{
    Context, Store,
    shape::*, material::*, object::Covered,
};
use clay::{
    scene::ListScene, view::ProjView,
    shape::*, material::*,
    process::{DefaultRenderer, DefaultPostproc},
    background::{GradientBackground as GradBg},
};
use clay_viewer::{Window};

// Here we declare our object - a combination of
// spherical shape and colored diffuse material
type MyObject = Covered<Sphere, Colored<Diffuse>>;

// Scene contains our objects and has gradient background
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
    let dims = (800, 600);

    let mut scene = ListScene::new(GradBg::new(
        Vector3::new(0.8, 0.8, 0.8), Vector3::new(0.2, 0.2, 0.2),
        Vector3::new(0.0, 0.0, 1.0),
    ));
    scene.add(
        Sphere::new(0.75, Vector3::new(-0.75, 0.0, 0.0))
        .cover(Diffuse {}.color_with(Vector3::new(0.3, 0.9, 0.3)))
    );
    scene.add(
        Sphere::new(1.0, Vector3::new(1.0, 0.0, 0.0))
        .cover(Diffuse {}.color_with(Vector3::new(0.3, 0.3, 0.9)))
    );

    let view = ProjView {
        pos: Vector3::new(0.0, -2.0, 0.0),
        ori: Matrix3::identity(),
    };

    let mut renderer = DefaultRenderer::<MyScene, MyView>::new(dims, scene, view).unwrap();
    let (mut worker, message) = renderer.create_worker(&context).unwrap();
    if message.len() > 0 {
        println!("render build log:\n{}", message);
    }

    let (mut postproc, message) = DefaultPostproc::builder().unwrap()
    .build(&context, dims).unwrap();
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
        renderer.view.pos = pos + Vector3::new(0.0, -2.0, 0.0);
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
