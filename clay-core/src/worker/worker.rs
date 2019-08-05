use std::{
    path::Path,
    marker::PhantomData,
};
use ocl::{self, prm};
use ocl_include::{MemHook, ListHook};
use crate::{
    Context,
    Scene, View,
    Screen,
    get_ocl_src,
};
use super::{Program};

pub struct Programs {
    pub render: Program,
    pub draw: Program,
}

pub struct Kernels {
    render: ocl::Kernel,
    draw: ocl::Kernel,
}

#[allow(dead_code)]
pub struct Worker<S: Scene, V: View> {
    programs: Programs,
    kernels: Kernels,
    queue: ocl::Queue,
    phantom: PhantomData<(S, V)>,
}

impl<S: Scene, V: View> Worker<S, V> {
    pub fn new(context: &Context) -> crate::Result<Self> {
        let queue = context.queue().clone();

        let render_prog = Program::new(
            &ListHook::builder()
            .add_hook(get_ocl_src())
            .add_hook(
                MemHook::builder()
                .add_file(&Path::new("__gen__/scene.h"), S::ocl_scene_code())?
                .add_file(&Path::new("__gen__/view.h"), V::ocl_view_code())?
                .build()
            )
            .build(),
            &Path::new("clay_core/render.c"),
        )?;
        let ocl_render_prog = render_prog.build(context)?;

        // build kernel
        let mut kb = ocl::Kernel::builder();
        kb.program(&ocl_render_prog)
        .name("render")
        .queue(queue.clone())
        .arg(prm::Int2::zero()) // screen size
        .arg(None::<&ocl::Buffer<prm::Float3>>) // color buffer
        .arg(None::<&ocl::Buffer<u32>>); // random
        V::args_def(&mut kb);
        S::args_def(&mut kb);
        let render_kernel = kb.build()?;

        // draw program
        let draw_prog = Program::new(&get_ocl_src(), &Path::new("clay_core/draw.c"))?;
        let ocl_draw_prog = draw_prog.build(context)?;
        let draw_kernel = ocl::Kernel::builder()
        .program(&ocl_draw_prog)
        .name("draw")
        .queue(queue.clone())
        .arg(prm::Int2::zero()) // screen size
        .arg(0i32) // passes
        .arg(None::<&ocl::Buffer<prm::Float3>>) // color buffer
        .arg(None::<&ocl::Buffer<u8>>) // screen
        .build()?;

        Ok(Self {
            programs: Programs { render: render_prog, draw: draw_prog },
            kernels: Kernels { render: render_kernel, draw: draw_kernel },
            queue, phantom: PhantomData,
        })
    }

    pub fn programs(&self) -> &Programs {
        &self.programs
    }

    pub fn render(
        &mut self,
        screen: &mut Screen,
        scene: &S,
        view: &V,
    ) -> crate::Result<()> {
        let kernel = &mut self.kernels.render;

        let dims = screen.dims();
        let dims = prm::Int2::new(dims.0 as i32, dims.1 as i32);
        kernel.set_arg(0, &dims)?;
        kernel.set_arg(1, screen.color_mut())?;
        kernel.set_arg(2, screen.random_mut())?;
        let mut i = 3;

        view.args_set(i, kernel)?;
        i += V::args_count();

        scene.args_set(i, kernel)?;
        //i += S::args_count();

        unsafe {
            kernel
            .cmd()
            .global_work_size(screen.dims())
            .enq()?;
        }

        self.queue.finish()?;

        screen.pass();

        let kernel = &mut self.kernels.draw;
        let dims = screen.dims();
        let dims = prm::Int2::new(dims.0 as i32, dims.1 as i32);
        kernel.set_arg(0, &dims)?;
        kernel.set_arg(1, &(screen.n_passes() as i32))?;
        kernel.set_arg(2, screen.color_mut())?;
        kernel.set_arg(3, screen.bytes_mut())?;

        unsafe {
            kernel
            .cmd()
            .global_work_size(screen.dims())
            .enq()?;
        }

        self.queue.finish()?;

        Ok(())
    }
}
