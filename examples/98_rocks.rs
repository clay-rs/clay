use std::{env, time::Duration};
use nalgebra::{Vector3, Rotation3, Matrix3};
use clay::{
    prelude::*,
    shape::*,
    material::*,
    object::*,
    scene::{TargetListScene, GradientBackground as GradBg},
    view::ProjectionView,
    process::{create_renderer, create_default_postproc},
    shape_select, material_select, material_combine,
};
use clay_viewer::{Window, Motion};
use clay_utils::{args, FrameCounter};

shape_select!(MyShape {
    P(TP=Parallelepiped),
    S(TS=Ellipsoid),
});
material_combine!(Glossy {
    reflect: Reflective,
    diffuse: Colored<Diffuse>,
});
material_combine!(Glowing {
    reflect: Reflective,
    diffuse: Colored<Luminous>,
});
material_select!(MyMaterial {
    D(TD=Colored<Diffuse>),
    G(TG=Glossy),
    F(TF=Glowing),
    L(TL=Colored<Luminous>),
});

// Here we declare our object - a combination of
// spherical shape and colored diffuse material
type MyObject = Covered<MyShape, MyMaterial>;

// Scene contains our objects and has gradient background
type MyScene = TargetListScene<MyObject, Sphere, GradBg>;
type MyView = ProjectionView;


fn main() -> clay::Result<()> {
    // Parse args to select OpenCL platform
    let context = args::parse(env::args())?;

    // Dimensions of the window
    let dims = (1280, 800);

    // Initialize the scene
    let mut scene = TargetListScene::new(GradBg::new(
        Vector3::new(0.1, 0.1, 0.3), Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    ));

    // Ground
    scene.add(MyShape::from(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(1000.0, 1000.0, 0.5)),
        Vector3::new(0.0, 0.0, -0.5),
    )).cover(MyMaterial::from(Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9)))));
    
    // Rocks
    let mut rocks = Vec::new();

    rocks.push((30.0, (30.0, 60.0), 1e3*Vector3::new(1.0, 0.5, 0.5)));
    rocks.push((100.0, (0.0, 200.0), 1e3*Vector3::new(0.5, 0.5, 1.0)));
    rocks.push((15.0, (-30.0, 30.0), 1e3*Vector3::new(0.5, 1.0, 0.5)));

    let rot = Rotation3::rotation_between(
        &Vector3::new(1.0, 1.0, 1.0),
        &Vector3::new(0.0, 0.0, 1.0),
    ).unwrap().matrix().clone();
    for (size, pos, color) in rocks {
        scene.add(MyShape::from(Parallelepiped::new(
            size*3.0f64.sqrt()/2.0*rot.clone(),
            Vector3::new(pos.0, pos.1, -0.5*size),
        )).cover(MyMaterial::from(Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9)))));
        let tsize = 0.025*size;
        scene.add_targeted(MyShape::from(Parallelepiped::new(
            tsize*3.0f64.sqrt()/2.0*rot*Matrix3::from_diagonal(&Vector3::new(0.5, 0.5, 2.0)),
            Vector3::new(pos.0, pos.1, 1.2*size + 2.0*tsize),
        )).cover(MyMaterial::from(Luminous {}.color_with(color))));
    }

    scene.add(MyShape::from(Ellipsoid::new(
        0.25*Matrix3::identity(),
        Vector3::new(1.0, 0.0, 0.25),
    )).cover(MyMaterial::from(Luminous {}.color_with(2.0*Vector3::new(0.8, 1.0, 0.8)))));

    scene.add(MyShape::from(Ellipsoid::new(
        0.4*Matrix3::identity(),
        Vector3::new(0.0, 1.0, 0.4),
    )).cover(MyMaterial::from(Glossy::new(
        (0.2, Reflective {}),
        (0.8, Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9))),
    ))));

    scene.add(MyShape::from(Parallelepiped::new(
        0.5*Matrix3::identity(),
        Vector3::new(-1.0, 0.0, 0.5),
    )).cover(MyMaterial::from(Glossy::new(
        (0.5, Reflective {}),
        (0.5, Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9))),
    ))));
    
    // Create view
    let view = ProjectionView::new(
        Vector3::new(2.0, 0.0, 1.0),
        Rotation3::face_towards(&-Vector3::new(-1.0, 0.0, 0.0), &Vector3::z_axis()),
    );

    // Create renderer and worker
    let mut renderer = create_renderer::<MyScene, MyView>().build(dims, scene, view)?;
    let (mut worker, _) = renderer.create_worker(&context)?;

    // Create dummy postprocessor
    let (mut postproc, _) = create_default_postproc().collect()?
    .build_default(&context, dims)?;

    // Create viewer window
    let mut window = Window::new(dims)?;
    // Capture mouse
    window.set_capture_mode(true);

    // Create motion controller
    let mut motion = Motion::new(renderer.view.pos, renderer.view.ori.clone());

    // Structure for performance measurement (optional)
    let mut fcnt = FrameCounter::new();

    // Main loop - repeatedly update view and render
    while !window.poll_with_handler(&mut motion)? {
        if motion.was_updated() {
            // Clear cumulative buffer
            worker.data_mut().buffer_mut().clear()?;
        }
        // Move to a new location
        let dt = window.state().frame_duration();
        motion.step(dt);

        // Update view location
        renderer.view.update(motion.pos(), motion.ori());
        renderer.view.fov = motion.fov;
        renderer.update_data(&context, worker.data_mut())?;

        // Render
        let n = worker.run_for(Duration::from_millis(20))?;
        // Print FPS (optional)
        fcnt.step_frame(dt, n);

        // Postprocess
        postproc.process_one(&worker.data().buffer())?;
        postproc.make_image()?;

        // Draw image to Window
        window.draw(postproc.image())?;
    }

    Ok(())
}
