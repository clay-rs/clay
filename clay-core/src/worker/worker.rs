use std::{
    path::Path,
    collections::HashSet,
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

pub struct Programs<P> {
    pub render: P,
    pub draw: P,
}

pub struct Kernels {
    render: ocl::Kernel,
    draw: ocl::Kernel,
}

pub struct WorkerBuilder<S: Scene, V: View> {
    programs: Programs<Program>,
    phantom: PhantomData<(S, V)>,
}

#[allow(dead_code)]
pub struct Worker<S: Scene, V: View> {
    programs: Programs<(Program, String)>,
    kernels: Kernels,
    queue: ocl::Queue,
    phantom: PhantomData<(S, V)>,
}

impl<S: Scene, V: View> WorkerBuilder<S, V> {
    pub fn programs(&self) -> &Programs<Program> {
        &self.programs
    }
}

impl<S: Scene, V: View> Worker<S, V> {
    pub fn builder() -> crate::Result<WorkerBuilder<S, V>> {
        let mut inst_cache = HashSet::<u64>::new();
        let render_prog = Program::new(
            &ListHook::builder()
            .add_hook(get_ocl_src())
            .add_hook(
                MemHook::builder()
                .add_file(&Path::new("__gen/scene.h"), S::source(&mut inst_cache))?
                .add_file(&Path::new("__gen/view.h"), V::source(&mut inst_cache))?
                .build()
            )
            .build(),
            &Path::new("clay_core/render.c"),
        )?;

        let draw_prog = Program::new(&get_ocl_src(), &Path::new("clay_core/draw.c"))?;

        Ok(WorkerBuilder {
            programs: Programs { render: render_prog, draw: draw_prog },
            phantom: PhantomData,
        })
    }
}

impl<S: Scene, V: View> WorkerBuilder<S, V> {
    pub fn build(self, context: &Context) -> crate::Result<Worker<S, V>> {
        let queue = context.queue().clone();

        let render_prog = self.programs.render;
        let ocl_render_prog = render_prog.build(context)?;

        // build kernel
        let mut kb = ocl::Kernel::builder();
        kb.program(&ocl_render_prog.0)
        .name("render")
        .queue(queue.clone())
        .arg(prm::Int2::zero()) // screen size
        .arg(None::<&ocl::Buffer<prm::Float3>>) // color buffer
        .arg(None::<&ocl::Buffer<u32>>); // random
        V::args_def(&mut kb);
        S::args_def(&mut kb);
        let render_kernel = kb.build()?;

        // draw program
        let draw_prog = self.programs.draw;
        let ocl_draw_prog = draw_prog.build(context)?;
        let draw_kernel = ocl::Kernel::builder()
        .program(&ocl_draw_prog.0)
        .name("draw")
        .queue(queue.clone())
        .arg(prm::Int2::zero()) // screen size
        .arg(0i32) // passes
        .arg(None::<&ocl::Buffer<prm::Float3>>) // color buffer
        .arg(None::<&ocl::Buffer<u8>>) // screen
        .build()?;

        Ok(Worker {
            programs: Programs {
                render: (render_prog, ocl_render_prog.1),
                draw: (draw_prog, ocl_draw_prog.1),
            },
            kernels: Kernels { render: render_kernel, draw: draw_kernel },
            queue, phantom: PhantomData,
        })
    }
}

impl<S: Scene, V: View> Worker<S, V> {
    pub fn programs(&self) -> &Programs<(Program, String)> {
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
