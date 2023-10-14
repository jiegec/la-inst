use indicatif::{ProgressIterator, ProgressStyle};
use la_inst::{inst_legal_ptrace, ProbeResult, inst_decode_binutils};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let style =
        ProgressStyle::with_template("{bar:40} {pos:>7}/{len:7} [{per_sec}] [ETA {eta}]").unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("x86settag.txt")
        .unwrap();
    for i in (0..8192).progress_with_style(style) {
        let imm1 = i >> 8;
        let imm2 = i & 255;
        let rd = 1;
        let inst = 0x00580000 | (imm1 << 5) | (imm2 << 10) | rd;
        writeln!(file, "{:?}", inst_decode_binutils(inst)).unwrap();

        let result = inst_legal_ptrace(inst, &[]).unwrap();
        write!(file, "x86settag $r1, {}, {}: ", imm1, imm2).unwrap();
        match result {
            ProbeResult::IllegalInstruction => todo!(),
            ProbeResult::SegmentationFault => todo!(),
            ProbeResult::BusError => todo!(),
            ProbeResult::RegisterUnchaged => writeln!(file, "unchanged").unwrap(),
            ProbeResult::BinaryTranslationException => writeln!(file, "exception").unwrap(),
            ProbeResult::RegisterChanged(info) => {
                writeln!(
                    file,
                    "rd ^= {}",
                    info.new.gpr[rd as usize] ^ info.old.gpr[rd as usize]
                )
                .unwrap();
            }
        }
    }
}
