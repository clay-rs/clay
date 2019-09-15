use std::{env, time::Duration};
use nalgebra::{Vector3, Rotation3, Matrix3};
use clay::{
    prelude::*,
    shape::*,
    material::*,
    object::*,
    scene::{TargetListScene, ConstantBackground as ConstBg},
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
material_select!(MyMaterial {
    D(TD=Colored<Diffuse>),
    R(TR=Reflective),
    G(TG=Glossy),
    L(TL=Colored<Luminous>),
});

// Here we declare our object - a combination of
// spherical shape and colored diffuse material
type MyObject = Covered<MyShape, MyMaterial>;

// Scene contains our objects and has gradient background
type MyScene = TargetListScene<MyObject, Sphere, ConstBg>;
type MyView = ProjectionView;


fn main() -> clay::Result<()> {
    // Parse args to select OpenCL platform
    let context = args::parse(env::args())?;

    // Dimensions of the window
    let dims = (1280, 800);

    // Initialize the scene
    let mut scene = TargetListScene::new(ConstBg::new(Vector3::new(1.0, 1.0, 3.0)));
    scene.set_max_depth(8);
    scene.set_target_prob(0.1);

    // Add room
    let size = (3.0, 3.0, 1.5); // room parameters
    let (wpos, wsize) = (0.5, (1.5, 1.2)); // window parameters
    let thc = 0.05; // thickness
    let mut parts = Vec::new();
    
    // ceil
    parts.push((Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(size.0 + 2.0*thc, size.1 + 2.0*thc, thc)),
        Vector3::new(0.0, 0.0, 2.0*size.2 + thc),
    ), Vector3::new(0.9, 0.9, 0.9)));
    // walls
    parts.push((Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(thc, size.1, size.2)),
        Vector3::new(-(size.0 + thc), 0.0, size.2),
    ), Vector3::new(0.1, 0.1, 1.0)));
    parts.push((Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(size.0 + 2.0*thc, thc, size.2)),
        Vector3::new(0.0, -(size.1 + thc), size.2),
    ), Vector3::new(1.0, 1.0, 0.1)));
    parts.push((Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(size.0 + 2.0*thc, thc, size.2)),
        Vector3::new(0.0, size.1 + thc, size.2),
    ), Vector3::new(0.1, 1.0, 0.1)));
    // last wall with window
    let mut wparts = Vec::new();
    wparts.push(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(thc, size.1, 0.5*wpos)),
        Vector3::new(size.0 + thc, 0.0, 0.5*wpos),
    ));
    wparts.push(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(thc, size.1, size.2 - wsize.1 - 0.5*wpos)),
        Vector3::new(size.0 + thc, 0.0, size.2 + wsize.1 + 0.5*wpos),
    ));
    wparts.push(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(thc, 0.5*(size.1 - wsize.0), wsize.1)),
        Vector3::new(size.0 + thc, 0.5*(size.1 + wsize.0), wpos + wsize.1),
    ));
    wparts.push(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(thc, 0.5*(size.1 - wsize.0), wsize.1)),
        Vector3::new(size.0 + thc, -0.5*(size.1 + wsize.0), wpos + wsize.1),
    ));
    // window cross
    wparts.push(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(thc, thc, wsize.1)),
        Vector3::new(size.0 + thc, 0.0, wpos + wsize.1),
    ));
    wparts.push(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(thc, wsize.0, thc)),
        Vector3::new(size.0 + thc, 0.0, wpos + wsize.1),
    ));
    for wp in wparts {
        parts.push((wp, Vector3::new(0.9, 0.9, 0.9)));
    }
    for (s, c) in parts {
        scene.add(MyShape::from(s).cover(MyMaterial::from(Diffuse {}.color_with(c))));
    }

    // floor
    scene.add(MyShape::from(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(size.0 + 2.0*thc, size.1 + 2.0*thc, thc)),
        Vector3::new(0.0, 0.0, -thc),
    )).cover(MyMaterial::from(Glossy::new(
        (0.1, Reflective {}),
        (0.9, Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9))),
    ))));

    // Bed
    scene.add(MyShape::from(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(0.8, 1.2, 0.3)),
        Vector3::new(-(size.0 - 0.8 - thc), size.1 - 1.2 - thc, 0.3 + thc),
    )).cover(MyMaterial::from(Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9)))));
    // Shelf
    scene.add(MyShape::from(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(0.8, 0.2, 1.0)),
        Vector3::new(-(size.0 - 0.8 - thc), -(size.1 - 0.2 - thc), 1.0),
    )).cover(MyMaterial::from(Diffuse {}.color_with(Vector3::new(0.9, 0.9, 0.9)))));
    // Mirror
    scene.add(MyShape::from(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(0.8, 0.01, 0.75)),
        Vector3::new(0.0, size.1 - 0.01, 1.25),
    )).cover(MyMaterial::from(Reflective {})));
    // Ball
    scene.add(MyShape::from(Ellipsoid::new(
        0.4*Matrix3::identity(),
        Vector3::new(-(size.0 - 0.4 - thc), -0.4*(size.1 - 0.4), 0.4),
    )).cover(MyMaterial::from(Reflective {})));
    // Table
    scene.add(MyShape::from(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(0.6, 0.6, 0.1)),
        Vector3::new(0.0, size.1 - 0.6, 0.2),
    )).cover(MyMaterial::from(Glossy::new(
        (0.1, Reflective {}),
        (0.9, Diffuse {}.color_with(Vector3::new(0.2, 0.1, 0.1))),
    ))));

    // Add ground
    scene.add(MyShape::from(Parallelepiped::new(
        Matrix3::from_diagonal(&Vector3::new(100.0, 100.0, 0.5)),
        Vector3::new(0.0, 0.0, -0.5 - 2.0*thc),
    )).cover(MyMaterial::from(Diffuse {}.color_with(Vector3::new(0.3, 1.0, 0.3)))));

    // Add light source
    let dist = 1e4;
    let lrad = 2e-2*dist;
    scene.add_targeted(MyShape::from(Ellipsoid::new(
        lrad*Matrix3::identity(), dist*Vector3::new(1.0, 0.2, 0.3),
    )).cover(MyMaterial::from(Luminous {}.color_with(1e5*Vector3::new(1.0, 1.0, 0.6)))));
    
    // Create view
    let mut view = ProjectionView::new(
        Vector3::new(2.0, 0.0, 1.0),
        Rotation3::face_towards(&-Vector3::new(-1.0, 0.0, 0.0), &Vector3::z_axis()),
    );
    view.fov = 1.2;

    // Create renderer and worker
    let mut renderer = create_renderer::<MyScene, MyView>().build(dims, scene, view)?;
    let (mut worker, _) = renderer.create_worker(&context)?;

    // Create dummy postprocessor
    let (mut postproc, _) = create_postproc().collect()?
    .build(&context, dims, LogFilter::new(-1.0, 2.0))?;

    // Create viewer window
    let mut window = Window::new(dims)?;
    // Capture mouse
    window.set_capture_mode(true);

    // Create motion controller
    let mut motion = Motion::new(renderer.view.pos, renderer.view.ori.clone());

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
        fcnt.step_frame(dt, n);

        // Postprocess
        postproc.process_one(&worker.data().buffer())?;
        postproc.make_image()?;

        // Draw image to Window
        window.draw(postproc.image())?;
    }

    Ok(())
}
