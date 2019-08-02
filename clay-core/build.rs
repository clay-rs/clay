use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;


fn main() {
    let ocl_src_dir = Path::new("ocl-src");

    let mut files = Vec::new();
    for entry in WalkDir::new(ocl_src_dir).into_iter().map(|e| e.unwrap()) {
        println!("cargo:rerun-if-changed={}", entry.path().display());
        if entry.file_type().is_file() {
            files.push(entry.into_path());
        }
    }
    let lines = files.into_iter().map(|path| {
        let mut content = String::new();
        File::open(&path).unwrap().read_to_string(&mut content).unwrap();
        format!(
            "\t(\"{}\", r###\"{}\"###),",
            path.strip_prefix(ocl_src_dir).unwrap().display(),
            content,
        )
    }).collect::<Vec<_>>();
    let text = [
        format!("const OCL_SRC_LIST: [(&'static str, &'static str); {}] = [", lines.len()),
        lines.join("\n"),
        "];".to_string(),
    ].join("\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut out_file = File::create(&out_dir.join("ocl_src_list.rs")).unwrap();
    
    out_file.write_all(text.as_bytes()).unwrap();
}
