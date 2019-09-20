use std::{
    env,
    io::Write,
    fs::{File, create_dir_all},
    time::Duration,
};
use nalgebra::{Vector3, Matrix3, Rotation3};
use clay::{
    prelude::*,
    shape::*,
    material::*,
    object::*,
    scene::{TargetListScene, GradientBackground as GradBg},
    view::ProjectionView,
    filter::{IdentityFilter},
    process::{create_renderer, create_postproc},
    shape_select, material_select, material_combine,
};
use clay_viewer::{Window, Motion};
use clay_utils::{args, FrameCounter};

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
type MyView = ProjectionView;


fn main() -> clay::Result<()> {
    // Parse args to select OpenCL platform
    let context = args::parse(env::args())?;
    let dims = (1280, 800);

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
            0.3*Matrix3::identity(),
            Vector3::new(1.0, 0.0, 0.3),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.2, Reflective {}),
            (0.8, Diffuse {}.color_with(Vector3::new(0.5, 0.5, 0.9))),
        )))
    );
    scene.add(
        MyShape::from(Ellipsoid::new(
            0.5*Matrix3::identity(),
            Vector3::new(0.0, 0.0, 0.5),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.1, Reflective {}),
            (0.9, Diffuse {}.color_with(Vector3::new(0.9, 0.5, 0.5))),
        )))
    );
    scene.add(
        MyShape::from(Ellipsoid::new(
            0.4*Matrix3::identity(),
            Vector3::new(0.5, 1.0, 0.4),
        ))
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vector3::new(0.5, 0.9, 0.5)),
        ))
    );

    let view = ProjectionView::new(
        Vector3::new(0.5, -2.0, 2.0),
        Rotation3::face_towards(&-Vector3::new(0.0, 1.0,-0.75), &Vector3::z_axis()),
    );

    let mut renderer = create_renderer::<MyScene, MyView>().build(dims, scene, view)?;
    let postproc_builder = create_postproc::<IdentityFilter>().collect()?;

    create_dir_all("./__gen_programs")?;
    for (name, prog) in [
        ("render.c", &renderer.program()),
        ("filter.c", &postproc_builder.program()),
    ].iter() {
        File::create(&format!("__gen_programs/{}", name))?
        .write_all(prog.source().as_bytes())?;
    }

    let (mut worker, message) = renderer.create_worker(&context)?;
    if message.len() > 0 {
        println!("render build log:\n{}", message);
    }

    let (mut postproc, message) = postproc_builder.build(&context, dims, IdentityFilter::new())?;
    if message.len() > 0 {
        println!("filter build log:\n{}", message);
    }

    let mut window = Window::new(dims)?;
    window.set_capture_mode(true);

    let mut motion = Motion::new(renderer.view.pos, renderer.view.ori.clone());

    // Structure for frame rate measurement (optional)
    let mut fcnt = FrameCounter::new_with_log(Duration::from_secs(2));

    // Main loop - repeatedly update view and render
    while !window.poll_with_handler(&mut motion)? {
        // Render
        let n = worker.run_for(Duration::from_millis(20))?;

        // Postprocess
        postproc.process_one(&worker.data().buffer())?;
        postproc.make_image()?;

        // Draw image to Window
        window.draw(&postproc.image())?;

        // Measure frame duration
        let dt = window.step_frame();

        // Check motion occured
        if motion.was_updated() {
            // Clear cumulative buffer
            worker.data_mut().buffer_mut().clear()?;

            // Move to a new location
            motion.step(dt);
            
            // Update view location
            renderer.view.update(motion.pos(), motion.ori());
            renderer.view.fov = motion.fov;
            renderer.update_data(&context, worker.data_mut())?;
        }

        // Count and print frame rate
        fcnt.step_frame(dt, n);
    }

    Ok(())
}
