use colored::Colorize;
use la_inst::{inst_assemble_binutils, inst_decode_binutils, inst_legal_ptrace};
use std::{fmt::Debug, mem::transmute_copy};

fn colored_output<T: Debug>(old: T, new: T) -> String {
    let old_s = format!("{:016x?}", old);
    let new_s = format!("{:016x?}", new);
    assert_eq!(old_s.len(), new_s.len());

    let mut res = String::new();
    res += "\n    OLD=";
    for (old_c, new_c) in old_s.chars().zip(new_s.chars()) {
        if old_c == new_c {
            res.push(old_c);
        } else {
            res += &format!("{}", old_c.to_string().red());
        }
    }

    res += "\n    NEW=";
    for (old_c, new_c) in old_s.chars().zip(new_s.chars()) {
        if old_c == new_c {
            res.push(new_c);
        } else {
            res += &format!("{}", new_c.to_string().red());
        }
    }
    res
}

fn examine(inst: u32) {
    let rd = inst & 0x1f;
    let rj = (inst >> 5) & 0x1f;
    let rk = (inst >> 10) & 0x1f;
    println!(
        "Inst: 0x{:08x}, rd = {}, rj = {}, rk = {}",
        inst, rd, rj, rk
    );
    println!("Binutils: {:?}", inst_decode_binutils(inst));
    match inst_legal_ptrace(inst, &[]).unwrap() {
        la_inst::ProbeResult::IllegalInstruction => println!("Ptrace: Illegal instruction"),
        la_inst::ProbeResult::SegmentationFault => println!("Ptrace: Segmentation fault"),
        la_inst::ProbeResult::BusError => println!("Ptrace: Bus error"),
        la_inst::ProbeResult::BinaryTranslationException => {
            println!("Ptrace: Binary Translation Error")
        }
        la_inst::ProbeResult::RegisterUnchaged => println!("Ptrace: Registers unchanged"),
        la_inst::ProbeResult::RegisterChanged(info) => {
            println!("Ptrace: Register changed");
            let mut changed = false;

            // gpr
            for i in 1..32 {
                if info.old.gpr[i] != info.new.gpr[i] {
                    println!(
                        "GPR {}: {}",
                        i,
                        colored_output(info.old.gpr[i], info.new.gpr[i])
                    );
                    changed = true;
                }
            }

            // lasx
            for i in 0..32 {
                if info.old.lasx[i] != info.new.lasx[i] {
                    println!(
                        "FPR {}: {}",
                        i,
                        colored_output(info.old.lasx[i], info.new.lasx[i])
                    );

                    let old_double_transmute: [f64; 4] =
                        unsafe { transmute_copy(&info.old.lasx[i]) };
                    let new_double_transmute: [f64; 4] =
                        unsafe { transmute_copy(&info.new.lasx[i]) };
                    println!(
                        "FPR (double) {}:\n    OLD={:?}\n    NEW={:?}",
                        i, old_double_transmute, new_double_transmute
                    );

                    let old_float_transmute: [f32; 8] =
                        unsafe { transmute_copy(&info.old.lasx[i]) };
                    let new_float_transmute: [f32; 8] =
                        unsafe { transmute_copy(&info.new.lasx[i]) };
                    println!(
                        "FPR (float) {}:\n    OLD={:?}\n    NEW={:?}",
                        i, old_float_transmute, new_float_transmute
                    );

                    changed = true;
                }
            }

            // lbt
            for i in 0..5 {
                if info.old.lbt[i] != info.new.lbt[i] {
                    println!(
                        "LBT {}: {}",
                        i,
                        colored_output(info.old.lbt[i], info.new.lbt[i])
                    );
                    changed = true;
                }
            }

            if info.old.fcc != info.new.fcc {
                println!(
                    "FCC: {}",
                    colored_output(info.old.fcc, info.new.fcc)
                );
                changed = true;
            }

            if info.old.fcsr != info.new.fcsr {
                println!(
                    "FCSR: {}",
                    colored_output(info.old.fcsr, info.new.fcsr)
                );
                changed = true;
            }

            // print rd, rj & rk
            if changed {
                println!("Possible inputs:");
                println!("rd = 0x{:016x}", info.old.gpr[rd as usize]);
                println!("xrd = {:016x?}", info.old.lasx[rd as usize]);
                println!("rj = 0x{:016x}", info.old.gpr[rj as usize]);
                println!("xrj = {:016x?}", info.old.lasx[rj as usize]);
                println!("rk = 0x{:016x}", info.old.gpr[rk as usize]);
                println!("xrk = {:016x?}", info.old.lasx[rk as usize]);
            }
        }
    }
}

fn main() {
    let mut assemble = false;
    for arg in std::env::args().skip(1) {
        if arg == "-s" {
            assemble = true;
            continue;
        }

        let inst = if assemble {
            inst_assemble_binutils(&arg).unwrap()
        } else {
            // replace spaces
            let arg = arg.replace(" ", "");
            if arg.len() == 8 {
                u32::from_str_radix(&arg, 16).unwrap()
            } else {
                u32::from_str_radix(&arg, 2).unwrap()
            }
        };

        examine(inst);
    }
}
