use std::{collections::HashMap, path::{Path, PathBuf}, process::Command};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use crate::config::*;

#[derive(Debug, Serialize, Deserialize)]
struct Stage {
    name: String,
    meta: HashMap<String, u32>,
    sections: HashMap<String, u32>
}

#[derive(Debug, Serialize, Deserialize)]
struct Layout {
    start: u32,
    end: u32,
    stages: Vec<Stage>
}

lazy_static! {
    static ref PREFIX: PathBuf = ROOT_PROJ.join("bootloader");
    static ref LAYOUT_CONF: PathBuf = PREFIX.join("layout.yaml");
}

pub fn build() -> Vec<PathBuf> {
    [
        "bios/shared/src/layout.rs.temp",
        "bios/stage_1/stage_1.ld.temp",
        "bios/stage_2/stage_2.ld.temp",
        "bios/stage_3/stage_3.ld.temp",
    ].map(|x| PREFIX.join(x))
     .map(|x| apply_template(&LAYOUT_CONF, &x));
    [
        "bios/stage_1",
        "bios/stage_2",
        "bios/stage_3",
    ].map(|x| PREFIX.join(x))
     .map(|x| build_subproject(&x, &x.join("target.json")))
     .to_vec()
}

fn apply_template(temp_path: &Path, apply_file: &PathBuf) {
    let temp = std::fs::read_to_string(temp_path).unwrap();
    let temp: Layout = serde_yaml::from_str(&temp).unwrap();
    
    let mut sub_map: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for stage in temp.stages {
        let mut key_map = HashMap::new();

        key_map.extend(stage.meta);
        key_map.extend(stage.sections);
        
        sub_map.insert(stage.name, key_map);
    }

    let src = std::fs::read_to_string(apply_file).unwrap();
    let reg = Handlebars::new();
    let res = reg.render_template(&src, &sub_map).unwrap();

    
    std::fs::write(apply_file.with_extension(""), res).unwrap();
}


fn build_subproject(subproj_dir: &PathBuf, target_triple: &PathBuf) -> PathBuf {
    // build
    let subproject_name = subproj_dir
        .file_stem()
        .expect("Couldn't get subproject name")
        .to_str()
        .expect("Subproject Name is not valid UTF-8");
    let target_triple_file = Path::new(&target_triple)
        .file_stem()
        .expect("Couldn't get target file stem");
    let target_dir = TARGET.join(&subproject_name);

    (!(Command::new(&*CARGO)
        .current_dir(&subproj_dir)
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

    binary_path
}
