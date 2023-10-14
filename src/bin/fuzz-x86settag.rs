use indicatif::{ProgressIterator, ProgressStyle};
use la_inst::{inst_legal_ptrace, ProbeResult, RegisterPreset};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let style =
        ProgressStyle::with_template("{bar:40} {pos:>7}/{len:7} [{per_sec}] [ETA {eta}]").unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("x86settag.txt")
        .unwrap();
    for i in (0..8192).progress_with_style(style) {
        let imm1 = i >> 8;
        let imm2 = i & 255;
        let rd = 1;
        let inst = 0x00580000 | (imm1 << 5) | (imm2 << 10) | rd;

        let value = 0xab16cdef;
        let result =
            inst_legal_ptrace(inst, &[RegisterPreset::GeneralRegister(rd as usize, value)])
                .unwrap();
        write!(file, "x86settag $r1, {}, {}: ", imm1, imm2).unwrap();
        match &result {
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

        // verify
        let mask = 1 << (imm2 & 0x7);
        let low = imm2 & 63;
        let mut should_throw = false;
        let mut expected_result = value;
        match imm1 % 8 {
            0 => {
                if (value & mask) == 0 {
                    expected_result |= mask;
                } else {
                    should_throw = true;
                }
            }
            1 => {
                if (value & mask) != 0 {
                    expected_result &= !mask;
                } else {
                    should_throw = true;
                }
            }
            2 => {
                if value & (1 << (low / 8)) == 0 {
                    should_throw = true;
                } else if (value & mask) == 0 {
                    should_throw = true;
                }
            }
            3 => {
                if value & (1 << (low / 8)) == 0 {
                    should_throw = true;
                } else if (value & mask) != 0 {
                    expected_result &= !mask;
                } else {
                    should_throw = true;
                }
            }
            4 => {
                if value & (1 << (low / 8)) == 0 {
                    should_throw = true;
                } else if (value & mask) != 0 {
                    expected_result &= !mask;
                    expected_result &= !(1 << (low / 8));
                } else {
                    should_throw = true;
                }
            }
            5 => {}
            6 => {}
            7 => {}
            _ => unreachable!(),
        }

        if should_throw {
            assert_eq!(result, ProbeResult::BinaryTranslationException);
        } else {
            match &result {
                ProbeResult::RegisterChanged(info) => {
                    assert_eq!(info.new.gpr[rd as usize], expected_result);
                }
                ProbeResult::RegisterUnchaged => {
                    assert_eq!(expected_result, value);
                }
                ProbeResult::BinaryTranslationException => {
                    assert!(should_throw);
                }
                _ => {
                    println!("Unknown result: {:?}", result);
                    assert!(false)
                }
            }
        }
    }
}
