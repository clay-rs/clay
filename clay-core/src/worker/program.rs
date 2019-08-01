use std::{
    path::Path,
    marker::PhantomData,
};
use regex::{Regex, RegexBuilder, Captures};
use ocl::{
    self,
    enums::{ProgramBuildInfo as Pbi, ProgramBuildInfoResult as Pbir},
};
use ocl_include;
use lazy_static::lazy_static;
use crate::{Context, Scene, View};


lazy_static!{
    static ref LOCATION: Regex = RegexBuilder::new(
        r#"^([^:\r\n]*):(\d*):(\d*):"#
    ).multi_line(true).build().unwrap();
}

pub struct Program<S: Scene, V: View> {
    source: String,
    index: ocl_include::Index,
    phantom: PhantomData<(S, V)>,
}

impl<S: Scene, V: View> Program<S, V> {
    pub fn new() -> crate::Result<Self> {
        let fs_hook = ocl_include::FsHook::new()
        .include_dir(&Path::new("../clay-core/ocl-src/"))?;

        let mem_hook = ocl_include::MemHook::new()
        .add_file(&Path::new("__gen__/scene.h"), S::ocl_scene_code())?
        .add_file(&Path::new("__gen__/view.h"), V::ocl_view_code())?;

        let hook = ocl_include::ListHook::new()
        .add_hook(mem_hook)
        .add_hook(fs_hook);

        let node = ocl_include::build(&hook, Path::new("main.c"))?;
        let (source, index) = node.collect();

        Ok(Self { source, index, phantom: PhantomData })
    }

    pub fn source(&self) -> String {
        self.source.clone()
    }

    pub fn build(&self, context: &Context) -> crate::Result<ocl::Program> {
        ocl::Program::builder()
        .devices(context.device())
        .source(self.source.clone())
        .build(context.context())
        .and_then(|p| {
            p.build_info(context.device().clone(), Pbi::BuildLog)
            .map(|pbi| match pbi {
                Pbir::BuildLog(s) => s,
                _ => unreachable!(),
            })
            .map(|log| {
                if log.len() > 0 {
                    println!("Build log: {}", log);
                }
                p
            })
            .map_err(|e| e.into())
        })
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
