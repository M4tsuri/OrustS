use std::{fs::File, io::{Read, IoSlice, Write}, path::Path};
use layout::*;

static MINEFEST_PATH: &str = "/Users/ctsinon/Projects/Orangs/bootloader";
static STAGE1_IMG: &str = "target/stage_1/target/release/stage_1.bin";
static STAGE2_IMG: &str = "target/stage_2/target/release/stage_2.bin";
static STAGE3_IMG: &str = "target/stage_3/target/release/stage_3.bin";

static TARGET_IMG: &str = "target/bootloader.bin";

fn main() {
    let minefest_path = Path::new(MINEFEST_PATH);
    let target_path = minefest_path.join(TARGET_IMG);

    let stage1_path = minefest_path.join(STAGE1_IMG);
    let stage2_path = minefest_path.join(STAGE2_IMG);
    let stage3_path = minefest_path.join(STAGE3_IMG);
    

    let mut stage1_file = File::open(stage1_path).unwrap();
    let mut stage2_file = File::open(stage2_path).unwrap();
    let mut stage3_file = File::open(stage3_path).unwrap();

    let mut stage1_tmp = vec![];
    let mut stage2_tmp = vec![];
    let mut stage3_tmp = vec![];

    stage1_file.read_to_end(&mut stage1_tmp).unwrap();
    stage2_file.read_to_end(&mut stage2_tmp).unwrap();
    stage3_file.read_to_end(&mut stage3_tmp).unwrap();

    stage1_tmp.resize(STAGE1_SIZE as usize, 0);
    stage2_tmp.resize(STAGE2_SIZE as usize, 0);
    stage3_tmp.resize(STAGE3_SIZE as usize, 0);

    let stage1 = IoSlice::new(stage1_tmp.as_slice());
    let stage2 = IoSlice::new(stage2_tmp.as_slice());
    let stage3 = IoSlice::new(stage3_tmp.as_slice());

    let mut res = File::create(target_path).unwrap();
    res.write_vectored(&[stage1, stage2, stage3]).unwrap();
    res.flush().unwrap();
}
