use std::{fs::File, io::{Read, Write}, path::Path};
use layout::*;

static MINEFEST_PATH: &str = "/Users/ctsinon/Projects/Orangs/bootloader";

static STAGES: [(&str, usize); 3] = [
    (&"target/stage_1/target/release/stage_1.bin", STAGE1_SIZE),
    (&"target/stage_2/target/release/stage_2.bin", STAGE2_SIZE),
    (&"target/stage_3/target/release/stage_3.bin", STAGE3_SIZE)
];

static TARGET_IMG: &str = "target/bootloader.bin";

fn main() {
    let minefest_path = Path::new(MINEFEST_PATH);
    let target_path = minefest_path.join(TARGET_IMG);

    let extract_stage = |stage_dir: &str, size: usize| {
        let stage_path = minefest_path.join(stage_dir);
        let mut stage_file = File::open(stage_path).unwrap();
        let mut tmp = vec![];
        stage_file.read_to_end(&mut tmp).unwrap();
        tmp.resize(size as usize, 0);
        tmp
    };

    let mut res = File::create(target_path).unwrap();
    for (dir, size) in STAGES {
        res.write(&extract_stage(dir, size)).unwrap();
    }
    res.flush().unwrap();
}
