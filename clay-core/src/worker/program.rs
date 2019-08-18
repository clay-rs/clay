use std::{
    path::Path,
};
use regex::{Regex, RegexBuilder, Captures};
use ocl::{
    self,
    enums::{ProgramBuildInfo as Pbi, ProgramBuildInfoResult as Pbir},
};
use ocl_include::{self, Hook};
use lazy_static::lazy_static;
use crate::{Context};


lazy_static!{
    static ref LOCATION: Regex = RegexBuilder::new(
        r#"^([^:\r\n]*):(\d*):(\d*):"#
    ).multi_line(true).build().unwrap();
}

pub struct Program {
    source: String,
    index: ocl_include::Index,
}

impl Program {
    pub fn new<H: Hook>(hook: &H, main: &Path) -> crate::Result<Self> {
        let node = ocl_include::build(hook, main)?;
        let (source, index) = node.collect();

        Ok(Self { source, index })
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
                caps[2].parse::<usize>().map_err(|_| ())
                .and_then(|line| {
                    // extra `- 1` is a workaround because ocl line numbers are shifted
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
