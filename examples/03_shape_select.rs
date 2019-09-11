use std::time::Duration;
use ocl::{Platform, Device};
use nalgebra::{Vector3, Rotation3, Matrix3};
use clay::{
    prelude::*,
    Context,
    shape::*,
    material::*,
    object::*,
    scene::{ListScene, GradientBackground as GradBg},
    view::ProjectionView,
    process::{create_renderer, create_default_postproc},
    shape_select,
};
use clay_viewer::{Window, Motion};

shape_select!(MyShape {
    P(TP=Parallelepiped),
    S(TS=Ellipsoid),
});

// Here we declare our object - a combination of
// spherical shape and colored diffuse material
type MyObject = Covered<MyShape, Colored<Diffuse>>;

// Scene contains our objects and has gradient background
type MyScene = ListScene<MyObject, GradBg>;
type MyView = ProjectionView;


fn main() -> clay::Result<()> {
    // Select default OpenCL platform and device
    let platform = Platform::default();
    let device = Device::first(platform)?;
    let context = Context::new(platform, device)?;

    // Dimensions of the window
    let dims = (1280, 800);

    // Initialize the scene
    let mut scene = ListScene::new(GradBg::new(
        Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    ));

    scene.add(
        MyShape::from(Ellipsoid::new(
            Matrix3::from_diagonal(&Vector3::new(0.8, 0.7, 0.4)),
            Vector3::new(0.0, 0.0, 0.4),
        ))
        .cover(Diffuse {}.color_with(Vector3::new(0.9, 0.3, 0.3)))
    );
    scene.add(
        MyShape::from(Ellipsoid::new(
            Matrix3::from_diagonal(&Vector3::new(0.4, 0.5, 0.2)),
            Vector3::new(0.0, 0.0, 1.0),
        ))
        .cover(Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9)))
    );
    scene.add(
        MyShape::from(Parallelepiped::new(
            0.4/3.0f64.sqrt()*Rotation3::rotation_between(
                &Vector3::new(1.0, 1.0, 1.0),
                &Vector3::new(0.0, 0.0, 1.0),
            ).unwrap().matrix().clone(),
            Vector3::new(0.0, 0.0, 1.6),
        ))
        .cover(Diffuse {}.color_with(Vector3::new(0.9, 0.3, 0.3)))
    );

    // Add ground
    scene.add(
        MyShape::from(Parallelepiped::new(
            Matrix3::from_diagonal(&Vector3::new(10.0, 10.0, 0.5)),
            Vector3::new(0.0, 0.0, -0.5),
        ))
        .cover(Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9)))
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

    // Main loop - repeatedly update view and render
    while !window.poll_with_handler(&mut motion)? {
        if motion.was_updated() {
            // Clear cumulative buffer
            worker.data_mut().buffer_mut().clear()?;
        }
        // Move to a new location
        motion.step(window.state().frame_duration());

        // Update view location
        renderer.view.update(motion.pos(), motion.ori());
        renderer.update_data(&context, worker.data_mut())?;

        // Render
        worker.run_for(Duration::from_millis(20))?;

        // Postprocess
        postproc.process_one(&worker.data().buffer())?;
        postproc.make_image()?;

        // Draw image to Window
        window.draw(postproc.image())?;
    }

    Ok(())
}
