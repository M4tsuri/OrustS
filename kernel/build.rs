// This is just a modified version of https://github.com/o8vm/krabs/blob/master/build.rs

#[macro_use]
extern crate lazy_static;
use std::fs::File;
use std::io::{Read, Write};

mod booloader {
    use std::{collections::HashMap, env, path::{Path, PathBuf}, process::Command};
    use handlebars::Handlebars;
    use llvm_tools::{LlvmTools, exe};
    use serde::{Deserialize, Serialize};

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
        pub static ref PREFIX: &'static Path = Path::new("../bootloader");
        pub static ref LAYOUT_CONF: PathBuf = PREFIX.join("layout.yaml");

        pub static ref CARGO: PathBuf = {
            let cargo_path = env::var("CARGO").expect("Missing CARGO environment variable");
            Path::new(&cargo_path).to_path_buf()
        };
        
        pub static ref OBJCOPY: PathBuf = {
            let llvm_tools = LlvmTools::new().expect("LLVM tools not found");
            llvm_tools
                .tool(&exe("llvm-objcopy"))
                .expect("llvm-objcopy not found")
        };

        pub static ref TARGET: PathBuf = {
            let cur = env::current_dir().unwrap();
            cur.join("target")
        };
    }

    pub fn build() -> Vec<PathBuf> {
        [
            "src/bios/shared/src/layout.rs.temp",
            "src/bios/stage_1/stage_1.ld.temp",
            "src/bios/stage_2/stage_2.ld.temp",
            "src/bios/stage_3/stage_3.ld.temp",
        ].map(|x| PREFIX.join(x))
         .map(|x| apply_template(&LAYOUT_CONF, &x));
        [
            "src/bios/stage_1",
            "src/bios/stage_2",
            "src/bios/stage_3",
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
        println!("cargo:rerun-if-changed={}", &target_triple.display());
        println!("cargo:rerun-if-changed={}", &subproj_dir.display());
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
}

fn read_to_bytes(f: &mut File) -> Vec<u8> {
    let mut res = vec![];
    f.read_to_end(&mut res).unwrap();
    res
}

fn main() {
    let target = booloader::PREFIX.join("bootloader.bin");
    let target = File::create(target).unwrap();

    booloader::build().iter()
        .map(|x| File::open(x).unwrap())
        .map(|mut x| read_to_bytes(&mut x))
        .fold(target, |mut f, data| {
            f.write(&data).unwrap();
            f
        });
}


