// This is just a modified version of https://github.com/o8vm/krabs/blob/master/build.rs

use llvm_tools::{exe, LlvmTools};
use std::{collections::HashMap, env};
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use handlebars::Handlebars;

fn main() {
    let templates = [
        "./src/bios/layout/src/lib.rs.temp",
        "./src/bios/stage_1/stage_1.ld.temp",
        "./src/bios/stage_2/stage_2.ld.temp",
        "./src/bios/stage_3/stage_3.ld.temp",
    ];

    for template in templates {
        apply_template("./layout.yaml", template);
    }
    
    build_all(&["src/bios/stage_1", "src/bios/stage_2", "src/bios/stage_3"]);
}

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

fn apply_template(temp_path: &str, apply_file: &str) {
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

    std::fs::write(apply_file.trim_end_matches(".temp"), res).unwrap();
}

fn build_all(stages: &[&str]) {
    let cargo_path = env::var("CARGO").expect("Missing CARGO environment variable");
    let cargo = Path::new(&cargo_path);
    let llvm_tools = LlvmTools::new().expect("LLVM tools not found");
    let objcopy = llvm_tools
        .tool(&exe("llvm-objcopy"))
        .expect("llvm-objcopy not found");

    let manifest_dir_path =
        env::var("CARGO_MANIFEST_DIR").expect("Missing CARGO_MANIFEST_DIR environment variable");
    let manifest_dir = Path::new(&manifest_dir_path);
    let current_dir = env::current_dir().expect("Couldn't get current directory");
    let target_dir_rel = manifest_dir.join("target");
    let target_dir = current_dir.join(target_dir_rel);

    let build_stage = |stage_dir: &str| {
        let stage_dir = manifest_dir.join(stage_dir);
        let stage_triple = stage_dir.join("target.json");
        build_subproject(
            &stage_dir,
            &stage_triple,
            &target_dir,
            &objcopy,
            &cargo,
        );
    };

    for stage in stages {
        build_stage(stage)
    }
}

fn build_subproject(
    subproject_dir: &Path,
    target_triple: &Path,
    root_target_dir: &Path,
    objcopy: &Path,
    cargo: &Path,
) {
    println!("cargo:rerun-if-changed={}", &target_triple.display());
    println!("cargo:rerun-if-changed={}", &subproject_dir.display());
    // build
    let subproject_name = subproject_dir
        .file_stem()
        .expect("Couldn't get subproject name")
        .to_str()
        .expect("Subproject Name is not valid UTF-8");
    let target_file = Path::new(&target_triple)
        .file_stem()
        .expect("Couldn't get target file stem");
    let target_dir = root_target_dir.join(&subproject_name);

    let mut build_cmd = Command::new(cargo);
    build_cmd.current_dir(&subproject_dir);
    build_cmd.arg("build").arg("--release");
    build_cmd.arg("-Zbuild-std=core,alloc");
    build_cmd.arg(format!("--target-dir={}", &target_dir.display()));
    build_cmd.arg("--target").arg(target_triple);
    let build_status = build_cmd.status().expect("Subcrate build failed!");
    assert!(build_status.success(), "Subcrate build failed!");

    // llvm-objcopy
    let object_dir = target_dir.join(&target_file).join("release");
    let object_path = object_dir.join(&subproject_name);
    let binary_path = object_dir.join(subproject_name.to_string() + ".bin");
    let mut objcopy_cmd = Command::new(objcopy);
    objcopy_cmd
        .arg("-I")
        .arg("elf32-i386")
        .arg("-O")
        .arg("binary");
    objcopy_cmd.arg(object_path);
    objcopy_cmd.arg(binary_path);
    let objcopy_status = objcopy_cmd.status().expect("Objcopy failed!");
    assert!(objcopy_status.success(), "Objcopy failed!");
}
