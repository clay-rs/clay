use std::{
    path::Path,
    marker::PhantomData,
};
use regex::{Regex, RegexBuilder, Captures};
use ocl;
use ocl_include;

use lazy_static::lazy_static;
use crate::{Context, Scene};


lazy_static!{
    static ref LOCATION: Regex = RegexBuilder::new(
        r#"^([^:\r\n]*):(\d*):(\d*):"#
    ).multi_line(true).build().unwrap();
}

pub struct Program<S: Scene> {
    source: String,
    index: ocl_include::Index,
    phantom: PhantomData<S>,
}

impl<S: Scene> Program<S> {
    fn gen_code() -> String {
        format!("{}\n{}",
            S::ocl_hit_code(),
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
                    S::ocl_hit_fn(),
                    "ray, ibuf, fbuf, dist, point, norm",
                ),
            ),
        )
    }

    pub fn new() -> crate::Result<Self> {
        let fs_hook = ocl_include::FsHook::new()
        .include_dir(&Path::new("../clay-core/ocl-src/"))?;

        let mem_hook = ocl_include::MemHook::new()
        .add_file(&Path::new("gen/worker.h"), Self::gen_code())?;

        let hook = ocl_include::ListHook::new()
        .add_hook(mem_hook)
        .add_hook(fs_hook);

        let node = ocl_include::build(&hook, Path::new("main.c"))?;
        let (source, index) = node.collect();

        Ok(Self { source, index, phantom: PhantomData })
    }

    pub fn build(&self, context: &Context) -> crate::Result<ocl::Program> {
        ocl::Program::builder()
        .devices(context.device())
        .source(self.source.clone())
        .build(context.context())
        .map_err(|e| {
            let message = LOCATION.replace_all(&e.to_string(), |caps: &Captures| -> String {
                if &caps[1] == "<kernel>" { Ok(()) } else { Err(()) }
                .and_then(|()| caps[2].parse::<usize>().map_err(|_| ()))
                .and_then(|line| {
                    // extra `- 1` is a workaround because ocl line numbers is shifted
                    self.index.search(line - 1 - 1).ok_or(())
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
        })
        .map_err(|e| e.into())
    }
}
