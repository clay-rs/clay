use std::{env, time::Duration};
use nalgebra::{Vector3, Rotation3, Matrix3};
use clay::{
    prelude::*,
    shape::*,
    material::*,
    object::*,
    scene::{ListScene, GradientBackground as GradBg},
    view::ProjectionView,
    filter::*,
    process::{create_renderer, create_postproc},
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
    diffuse: Reflective,
    reflect: Colored<Diffuse>,
    luminous: Colored<Luminous>,
});
material_select!(MyMaterial {
    D(TD=Colored<Diffuse>),
    R(TR=Glossy),
    G(TG=Glowing),
    L(TL=Colored<Luminous>),
});

// Here we declare our object - a combination of
// spherical shape and colored diffuse material
type MyObject = Covered<MyShape, MyMaterial>;

// Scene contains our objects and has gradient background
type MyScene = ListScene<MyObject, GradBg>;
type MyView = ProjectionView;


fn main() -> clay::Result<()> {
    // Parse args to select OpenCL platform
    let context = args::parse(env::args())?;

    // Dimensions of the window
    let dims = (1280, 800);

    // Initialize the scene
    let mut scene = MyScene::new(GradBg::new(
        2e-1*Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    ));
    scene.set_max_depth(6);

    scene.add(
        MyShape::from(Parallelepiped::new(
            Matrix3::from_diagonal(&Vector3::new(0.8, 1.6, 0.8)),
            Vector3::new(-2.0, 0.0, 0.8),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.95, Reflective {}),
            (0.05, Diffuse {}.color_with(Vector3::new(1.0, 1.0, 1.0))),
        )))
    );
    scene.add(
        MyShape::from(Ellipsoid::new(
            0.4*Matrix3::identity(),
            Vector3::new(0.0, 1.0, 0.4),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.2, Reflective {}),
            (0.8, Diffuse {}.color_with(Vector3::new(1.0, 0.1, 0.1))),
        )))
    );
    scene.add(
        MyShape::from(Ellipsoid::new(
            0.3*Matrix3::identity(),
            Vector3::new(0.0, -1.0, 0.3),
        ))
        .cover(MyMaterial::from(
            Diffuse {}.color_with(Vector3::new(0.3, 0.1, 0.9))
        ))
    );
    scene.add(
        MyShape::from(Ellipsoid::new(
            0.25*Matrix3::identity(),
            Vector3::new(0.0, 0.0, 0.25),
        ))
        .cover(MyMaterial::from(
            Luminous {}.color_with(20.0*Vector3::new(1.0, 1.0, 0.5)),
        ))
    );
    scene.add(
        MyShape::from(Ellipsoid::new(
            0.25*Matrix3::identity(),
            Vector3::new(1.0, 0.0, 0.25),
        ))
        .cover(MyMaterial::from(Glowing::new(
            (0.1, Reflective {}),
            (0.6, Diffuse {}.color_with(Vector3::new(1.0, 1.0, 1.0))),
            (0.3, Luminous {}.color_with(2.0*Vector3::new(0.1, 1.0, 0.1))),
        )))
    );

    scene.add(
        MyShape::from(Parallelepiped::new(
            Matrix3::from_diagonal(&Vector3::new(5.0, 5.0, 0.5)),
            Vector3::new(0.0, 0.0, -0.5),
        ))
        .cover(MyMaterial::from(Glossy::new(
            (0.1, Reflective {}),
            (0.9, Diffuse {}.color_with(Vector3::new(1.0, 1.0, 1.0))),
        )))
    );

    // Create view
    let view = ProjectionView::new(
        Vector3::new(2.0, -2.0, 2.0),
        Rotation3::face_towards(&-Vector3::new(-1.0, 0.8, -0.8), &Vector3::z_axis()),
    );

    // Create renderer and worker
    let mut renderer = create_renderer::<MyScene, MyView>().build(dims, scene, view)?;
    let (mut worker, _) = renderer.create_worker(&context)?;

    // Create dummy postprocessor
    let (mut postproc, _) = create_postproc().collect()?
    .build(&context, dims, LogFilter::new(-4.0, 2.0))?;

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
