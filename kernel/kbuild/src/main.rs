#[macro_use]
extern crate lazy_static;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
extern crate clap;
use clap::{Arg, value_t};

mod bootloader;
mod kernel;
mod config;


fn read_to_bytes(path: &PathBuf) -> Vec<u8> {
    let mut res = vec![];
    let mut f = File::open(path).unwrap();
    f.read_to_end(&mut res).unwrap();
    res
}


fn build(target: &Path) {
    let target = File::create(target).unwrap();
    
    bootloader::build()
        .iter()
        .chain(kernel::build().iter())
        .map(|x| read_to_bytes(x))
        .fold(target, 
            |mut f, bin| {
                f.write(&bin).unwrap(); 
                f 
            }
        );
}

fn run(target: &Path) {
    println!("here");
    if !target.is_file() {
        build(target);
    }
    // qemu-system-i386 -d int -no-reboot -drive format=raw,index=0,media=disk,file=bootloader.bin -vga std
    Command::new("qemu-system-i386")
        .args(["-d", "int"])
        .arg("-no-reboot")
        .arg("-drive")
        .arg(&format!("format=raw,index=0,media=disk,file={}", target.to_str().unwrap()))
        .args(["-vga", "std"])
        .spawn().unwrap();
}

enum Choice {
    Build,
    Run,
}

impl FromStr for Choice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "build" => Ok(Self::Build),
            "run" => Ok(Self::Run),
            _ => Err("no match")
        }
    }
}

fn main() {
    let matches = clap::App::new("kbuikd")
        .version("1.0.0")
        .arg(Arg::from_usage("<type> 'The type to use'")
            .possible_values(&["build", "run"]))
        .get_matches();

    let ty = value_t!(matches, "type", Choice)
        .unwrap_or_else(|e| e.exit());

    match ty {
        Choice::Build => build(&kernel::PREFIX.join("target").join("orusts")),
        Choice::Run => run(&kernel::PREFIX.join("target").join("orusts"))
    }

    println!("Build Done.");
}
