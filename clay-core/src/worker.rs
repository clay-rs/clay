use std::{
    path::Path,
    marker::PhantomData,
};
use regex::{Regex, RegexBuilder, Captures};
use ocl::{self, prm};
use ocl_include;
use vecmat::{
    vec::*,
    mat::*,
};
use crate::{
    Geometry,
    Context, Screen,
    buffer::ObjectBuffer,
};
use lazy_static::lazy_static;


lazy_static!{
    static ref LOCATION: Regex = RegexBuilder::new(
        r#"^([^:\r\n]*):(\d*):(\d*):"#
    ).multi_line(true).build().unwrap();
}


#[allow(dead_code)]
pub struct Worker<T: Geometry> {
    kernel: ocl::Kernel,
    queue: ocl::Queue,
    phantom: PhantomData<T>,
}

impl<T: Geometry> Worker<T> {
    fn ocl_code() -> String {
        format!("{}\n{}",
            T::ocl_hit_code(),
            format!(
                "bool hit({}) {{\n\t{}\n}}\n",
                format!("{}, {}, {}, {}, {}, {}",
                    "Ray ray",
                    "__global int *ibuf",
                    "__global float *fbuf",
                    "float *dist",
                    "float3 *point",
                    "float3 *norm",
                ),
                format!("return {}({});",
                    T::ocl_hit_fn(),
                    "ray, ibuf, fbuf, dist, point, norm",
                ),
            ),
        )
    }

    pub fn new(context: &Context) -> crate::Result<Self> {
        let queue = context.queue().clone();

        // load source
        let fs_hook = ocl_include::FsHook::new()
        .include_dir(&Path::new("../clay-core/ocl-src/"))?;

        let mem_hook = ocl_include::MemHook::new()
        .add_file(&Path::new("gen/worker.h"), Self::ocl_code())?;

        let hook = ocl_include::ListHook::new()
        .add_hook(mem_hook)
        .add_hook(fs_hook);

        let node = ocl_include::build(&hook, Path::new("main.c"))?;
        let (src, index) = node.collect();

        // build program
        let program = ocl::Program::builder()
        .devices(context.device())
        .source(src)
        .build(context.context())
        .map_err(|e| {
            let message = LOCATION.replace_all(&e.to_string(), |caps: &Captures| -> String {
                if &caps[1] == "<kernel>" { Ok(()) } else { Err(()) }
                .and_then(|()| caps[2].parse::<usize>().map_err(|_| ()))
                .and_then(|line| {
                    index.search(line - 1 - 1 /* workaround */).ok_or(())
                })
                .and_then(|(path, local_line)| {
                    Ok(format!(
                        "{}:{}:{}:",
                        path.to_string_lossy(),
                        local_line,
                        &caps[3],
                    ))
                })
                .unwrap_or(caps[0].to_string())
            }).into_owned();
            ocl::Error::from(ocl::core::Error::from(message))
        })?;

        // build kernel
        let kernel = ocl::Kernel::builder()
        .program(&program)
        .name("fill")
        .queue(queue.clone())
        .arg(&prm::Int2::zero())
        .arg(None::<&ocl::Buffer<u8>>)
        .arg(&prm::Float3::zero())
        .arg(&prm::Float16::zero())
        .arg(None::<&ocl::Buffer<i32>>)
        .arg(None::<&ocl::Buffer<f32>>)
        .arg(&0i32)
        .arg(&0i32)
        .arg(&0i32)
        .build()?;

        Ok(Self { kernel, queue, phantom: PhantomData })
    }

    pub fn render(&self, screen: &mut Screen, pos: Vec3<f64>, map: Mat3<f64>, objects: &ObjectBuffer<T>) -> crate::Result<()> {
        let dims = screen.dims();
        let dims = prm::Int2::new(dims.0 as i32, dims.1 as i32);
        self.kernel.set_arg(0, &dims)?;
        self.kernel.set_arg(1, screen.buffer_mut())?;

        let mapf = map.map(|x| x as f32);
        let mut map16 = [0f32; 16];
        map16[0..3].copy_from_slice(&mapf.row(0).data);
        map16[4..7].copy_from_slice(&mapf.row(1).data);
        map16[8..11].copy_from_slice(&mapf.row(2).data);
        self.kernel.set_arg(2, &prm::Float3::from(pos.map(|e| e as f32).data))?;
        self.kernel.set_arg(3, &prm::Float16::from(map16))?;

        self.kernel.set_arg(4, objects.buffer_int())?;
        self.kernel.set_arg(5, objects.buffer_float())?;
        self.kernel.set_arg(6, T::size_int() as i32)?;
        self.kernel.set_arg(7, T::size_float() as i32)?;
        self.kernel.set_arg(8, objects.count() as i32)?;

        unsafe {
            self.kernel
            .cmd()
            .global_work_size(screen.dims())
            .enq()?;
        }

        Ok(())
    }
}
