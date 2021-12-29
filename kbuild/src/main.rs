mod bootloader;
mod kernel;
mod config;

#[macro_use]
extern crate lazy_static;
extern crate clap;

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
    str::FromStr
};
use clap::{Arg, value_t};
use config::ROOT_PROJ;




fn read_to_bytes(path: &PathBuf) -> Vec<u8> {
    let mut res = vec![];
    let mut f = File::open(path).unwrap();
    f.read_to_end(&mut res).unwrap();
    res
}


fn build(target: &Path) {
    let target = File::create(target).unwrap();
    let mut wrote_bytes = 0;
    
    let mut f = bootloader::build()
        .iter()
        .chain(kernel::build().iter())
        .map(|x| read_to_bytes(x))
        .fold(target, |mut f, bin| {
            wrote_bytes += bin.len();
            println!("Size is {}", bin.len());
            f.write(&bin).unwrap(); 
            f 
        });
    
    let padding = vec![0; (1 << 9) - (((1 << 9) - 1) & wrote_bytes)];
    f.write(&padding).unwrap();
}

fn run(target: &Path) {
    if !target.is_file() {
        build(target);
    }
    // qemu-system-i386 -d int -no-reboot -drive format=raw,index=0,media=disk,file=bootloader.bin -vga std
    Command::new("qemu-system-i386")
        .args(["-d", "int"])
        .arg("-no-reboot")
        .arg("-drive")
        .arg(&format!("format=raw,index=0,if=ide,media=disk,file={}", target.to_str().unwrap()))
        .args(["-vga", "std"])
        .spawn().unwrap();
}

fn debug(target: &Path) {
    if !target.is_file() {
        build(target);
    }
    // qemu-system-i386 -d int -no-reboot -drive format=raw,index=0,media=disk,file=bootloader.bin -vga std
    Command::new("qemu-system-i386")
        .args(["-d", "int"])
        .arg("-no-reboot")
        .arg("-drive")
        .arg(&format!("format=raw,index=0,if=ide,media=disk,file={}", target.to_str().unwrap()))
        .args(["-vga", "std", "-s", "-S"])
        .spawn().unwrap();
}

enum Choice {
    Build,
    Run,
    Debug
}

impl FromStr for Choice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "build" => Ok(Self::Build),
            "run" => Ok(Self::Run),
            "debug" => Ok(Self::Debug),
            _ => Err("no match")
        }
    }
}

fn main() {
    let matches = clap::App::new("kbuikd")
        .version("1.0.0")
        .arg(Arg::from_usage("<type> 'The type to use'")
            .possible_values(&["build", "run", "debug"]))
        .get_matches();

    let ty = value_t!(matches, "type", Choice)
        .unwrap_or_else(|e| e.exit());

    match ty {
        Choice::Build => build(&ROOT_PROJ.join("target").join("orusts")),
        Choice::Run => run(&ROOT_PROJ.join("target").join("orusts")),
        Choice::Debug => debug(&ROOT_PROJ.join("target").join("orusts"))
    }

    println!("Build Done.");
}
