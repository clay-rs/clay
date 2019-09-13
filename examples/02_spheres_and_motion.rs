use std::{env, time::Duration};
use nalgebra::{Vector3, Rotation3};
use clay::{
    prelude::*,
    shape::*,
    material::*,
    object::*,
    scene::{ListScene, GradientBackground as GradBg},
    view::ProjectionView,
    process::{create_renderer, create_default_postproc},
};
use clay_viewer::{Window, Motion};
use clay_utils::{args, FrameCounter};


// Here we declare our object - a combination of
// spherical shape and colored diffuse material
type MyObject = Covered<Sphere, Colored<Diffuse>>;

// Scene contains our objects and has gradient background
type MyScene = ListScene<MyObject, GradBg>;
type MyView = ProjectionView;


fn main() -> clay::Result<()> {
    // Parse args to select OpenCL platform
    let context = args::parse(env::args())?;

    // Dimensions of the window
    let dims = (1280, 800);

    // Initialize the scene
    let mut scene = ListScene::new(GradBg::new(
        Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    ));

    // Add two spheres to the scene
    scene.add(
        Sphere::new(0.75, Vector3::new(-0.75, 0.0, 0.0))
        .cover(Diffuse {}.color_with(Vector3::new(0.4, 1.0, 0.4)))
    );
    scene.add(
        Sphere::new(1.0, Vector3::new(1.0, 0.0, 0.0))
        .cover(Diffuse {}.color_with(Vector3::new(0.4, 0.4, 1.0)))
    );

    // Create view
    let view = ProjectionView::new(
        Vector3::new(0.25, -3.0, 0.0),
        Rotation3::face_towards(&-Vector3::new(0.0, 1.0, 0.0), &Vector3::z_axis()),
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
