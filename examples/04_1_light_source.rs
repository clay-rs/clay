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
    shape_select, material_select,
};
use clay_viewer::{Window, Motion};
use clay_utils::{args, FrameCounter};


shape_select!(MyShape {
    P(TP=Parallelepiped),
    S(TS=Ellipsoid),
});
material_select!(MyMaterial {
    D(TD=Colored<Diffuse>),
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
        Vector3::new(0.1, 0.1, 0.2), Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    ));
    scene.set_max_depth(4);

    // Add complex shape
    let mut shapes = Vec::new();
    let size = 1.0;
    let fill = 0.5;
    for i in 0..4 {
        let (x, y) = (2.0*((i % 2) as f64) - 1.0, 2.0*((i / 2) as f64) - 1.0);
        shapes.push(Parallelepiped::new(
            Matrix3::from_diagonal(&Vector3::new(fill/3.0, fill/3.0, 1.0))*size,
            (Vector3::new(x, y, 0.0)*(1.0 - fill/3.0) + Vector3::new(0.0, 0.0, 1.0))*size,
        ));
        shapes.push(Parallelepiped::new(
            Matrix3::from_diagonal(&Vector3::new(fill, 3.0 - 2.0*fill, fill))*size/3.0,
            (Vector3::new(x, 0.0, y)*(1.0 - fill/3.0) + Vector3::new(0.0, 0.0, 1.0))*size,
        ));
        shapes.push(Parallelepiped::new(
            Matrix3::from_diagonal(&Vector3::new(3.0 - 2.0*fill, fill, fill))*size/3.0,
            (Vector3::new(0.0, x, y)*(1.0 - fill/3.0) + Vector3::new(0.0, 0.0, 1.0))*size,
        ));
    }
    for p in shapes {
        scene.add(MyShape::from(p).cover(
            MyMaterial::from(Diffuse {}.color_with(Vector3::new(0.3, 0.3, 0.9)))
        ));
    }

    // Add ground
    scene.add(
        MyShape::from(Parallelepiped::new(
            Matrix3::from_diagonal(&Vector3::new(10.0, 10.0, 0.5)),
            Vector3::new(0.0, 0.0, -0.5),
        ))
        .cover(MyMaterial::from(Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9))))
    );

    // Add light sources
    scene.add_targeted(
        MyShape::from(Ellipsoid::new(
            1.0*Matrix3::identity(), 10.0*Vector3::new(4.0, 6.0, 8.0),
        ))
        .cover(MyMaterial::from(Luminous {}.color_with(2e4*Vector3::new(1.0, 1.0, 0.8))))
    );
    
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
