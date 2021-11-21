use std::{env, path::PathBuf, path::Path};
use llvm_tools::{LlvmTools, exe};

lazy_static! {
    pub static ref ROOT_PROJ: &'static Path = Path::new("/Users/ctsinon/Projects/Orangs/OrustS/");
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
    pub static ref TARGET: PathBuf = ROOT_PROJ.join("target");
}