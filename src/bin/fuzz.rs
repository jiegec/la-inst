use indicatif::{ProgressIterator, ProgressStyle};
use la_inst::{inst_discovered, inst_legal_binutils, inst_legal_ptrace, ProbeResult};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let style =
        ProgressStyle::with_template("{bar:40} {pos:>7}/{len:7} [{per_sec}] [ETA {eta}]").unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .open("mismatch.txt")
        .unwrap();
    let max = u32::MAX >> 10;
    for i in (0..max).progress_with_style(style) {
        let inst = i << 10;
        if !inst_legal_binutils(inst) && !inst_discovered(inst) {
            // illegal instruction by binutils
            let result = inst_legal_ptrace(inst, &[]).unwrap();
            if result != ProbeResult::IllegalInstruction {
                println!("Found hidden instruction: 0x{:08x}", inst);
                writeln!(file, "Mismatch: {:08x}", inst).unwrap();
            }
        }
    }
}
