use la_inst::{inst_decode_binutils, inst_legal_ptrace};

fn examine(inst: u32) {
    println!(
        "Inst: 0x{:08x}, rd = {}, rj = {}, rk = {}",
        inst,
        inst & 0x1f,
        (inst >> 5) & 0x1f,
        (inst >> 10) & 0x1f
    );
    println!("Binutils: {:?}", inst_decode_binutils(inst));
    match inst_legal_ptrace(inst).unwrap() {
        la_inst::ProbeResult::IllegalInstruction => println!("Ptrace: Illegal instruction"),
        la_inst::ProbeResult::SegmentationFault => println!("Ptrace: Segmentation fault"),
        la_inst::ProbeResult::BusError => println!("Ptrace: Bus error"),
        la_inst::ProbeResult::RegisterUnchaged => println!("Ptrace: Registers unchanged"),
        la_inst::ProbeResult::RegisterChanged(info) => {
            println!("Ptrace: Register changed");

            // gpr
            for i in 1..32 {
                if info.old.gpr[i] != info.new.gpr[i] {
                    println!(
                        "GPR {}: OLD=0x{:016x} NEW=0x{:016x}",
                        i, info.old.gpr[i], info.new.gpr[i]
                    );
                }
            }

            // lasx
            for i in 0..32 {
                if info.old.lasx[i] != info.new.lasx[i] {
                    println!(
                        "FPR {}: OLD=0x{:016x?} NEW=0x{:016x?}",
                        i, info.old.lasx[i], info.new.lasx[i]
                    );
                }
            }
        }
    }
}

fn main() {
    for arg in std::env::args().skip(1) {
        examine(u32::from_str_radix(&arg, 16).unwrap());
    }
}
