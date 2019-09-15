use std::time::Duration;
use ocl::{Platform, Device};
use nalgebra::{Vector3, Rotation3};
use clay::{
    Context,
    shape::*,
    material::*,
    object::*,
    scene::{ListScene, GradientBackground as GradBg},
    view::ProjectionView,
    process::{create_renderer, create_default_postproc},
};
use clay_viewer::{Window};


// Here we declare our object - a combination of
// spherical shape and colored diffuse material
type MyObject = Covered<Sphere, Colored<Diffuse>>;

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
    let renderer = create_renderer::<MyScene, MyView>().build(dims, scene, view)?;
    let (mut worker, _) = renderer.create_worker(&context)?;

    // Create dummy postprocessor
    let (mut postproc, _) = create_default_postproc().collect()?
    .build_default(&context, dims)?;

    // Create viewer window
    let mut window = Window::new(dims)?;

    // Repeatedly trace rays and collect statictics
    // that subsequently reduces image noise
    while !window.poll()? {
        // Render scene
        worker.run_for(Duration::from_millis(20))?;
        // Postprocess
        postproc.process_one(&worker.data().buffer())?;
        // Make image
        postproc.make_image()?;
        // Draw image on window
        window.draw(postproc.image())?;
    }

    Ok(())
}
