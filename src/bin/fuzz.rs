use indicatif::{ProgressIterator, ProgressStyle};
use la_inst::{inst_decode_binutils, inst_legal_ptrace, ProbeResult};
use rand::Rng;

fn main() {
    let style = ProgressStyle::with_template("{bar:40} {pos:>7}/{len:7} [{per_sec}] ").unwrap();
    for _ in (0..10000).progress_with_style(style) {
        let mut rng = rand::thread_rng();
        let inst: u32 = rng.gen();
        if inst_decode_binutils(inst).unwrap().is_none() {
            // illegal instruction by binutils
            let result = inst_legal_ptrace(inst).unwrap();
            if result != ProbeResult::IllegalInstruction {
                println!("Found hidden instruction: 0x{:08x}", inst);
            }
        }
    }
}
