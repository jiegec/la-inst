use la_inst::{inst_decode_binutils, inst_legal_ptrace};

fn examine(inst: u32) {
    println!("Inst: 0x{:08x}", inst);
    println!("Binutils: {:?}", inst_decode_binutils(inst));
    println!("Ptrace: {:?}", inst_legal_ptrace(inst));
}

fn main() {
    examine(0x02ffc063);
    examine(0x38578bbb);
}
