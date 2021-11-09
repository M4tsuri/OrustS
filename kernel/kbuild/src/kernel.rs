use std::{path::{Path, PathBuf}, process::Command};
use crate::config::*;

lazy_static! {
    pub static ref PREFIX: PathBuf = ROOT_PROJ.join("kernel");
}

pub fn build() -> Vec<PathBuf> {
    let target_triple = PREFIX.join("target.json");
    // build
    let subproject_name = PREFIX
        .file_stem()
        .expect("Couldn't get subproject name")
        .to_str()
        .expect("Subproject Name is not valid UTF-8");
    let target_triple_file = Path::new(&target_triple)
        .file_stem()
        .expect("Couldn't get target file stem");
    let target_dir = TARGET.join(&subproject_name);

    (!(Command::new(&*CARGO)
        .current_dir(&*PREFIX)
        .arg("build")
        .arg("--release")
        .arg("-Zbuild-std=core,alloc")
        .arg(format!("--target-dir={}", &target_dir.display()))
        .arg("--target")
        .arg(target_triple.file_name().unwrap())
        .status()
        .expect("Subcrate build failed!")
        .success())).then(|| panic!("Subcrate build failed!"));

    // llvm-objcopy
    let object_dir = target_dir.join(&target_triple_file).join("release");
    let object_path = object_dir.join(&subproject_name);
    let binary_path = object_dir.join(subproject_name.to_string() + ".bin");
    (!(Command::new(&*OBJCOPY)
        .arg("-I")
        .arg("elf32-i386")
        .arg("-O")
        .arg("binary")
        .arg(&object_path)
        .arg(&binary_path)
        .status()
        .expect("Objcopy failed!")
        .success())).then(|| panic!("Objcopy failed!"));

    vec![binary_path]
}