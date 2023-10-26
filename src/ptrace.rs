use std::mem::size_of;

fn read_regs<T>(pid: libc::c_int, regs: &mut T, core_note: libc::c_int) {
    let iovec: libc::iovec = libc::iovec {
        iov_base: regs as *mut T as *mut libc::c_void,
        iov_len: size_of::<T>(),
    };
    assert_eq!(
        unsafe { libc::ptrace(libc::PTRACE_GETREGSET, pid, core_note, &iovec) },
        0
    );
}

pub fn read_gpr(pid: libc::c_int) -> libc::user_regs_struct {
    let mut regs: libc::user_regs_struct = libc::user_regs_struct {
        regs: [0; 32],
        orig_a0: 0,
        csr_era: 0,
        csr_badv: 0,
        reserved: [0; 10],
    };
    read_regs(pid, &mut regs, libc::NT_PRSTATUS);
    // r0 is always zero
    regs.regs[0] = 0;

    regs
}

pub fn read_fpr(pid: libc::c_int) -> libc::user_fp_struct {
    let mut regs: libc::user_fp_struct = libc::user_fp_struct {
        fpr: [0; 32],
        fcc: 0,
        fcsr: 0,
    };
    read_regs(pid, &mut regs, libc::NT_PRFPREG);
    regs
}

const NT_LOONGARCH_LSX: libc::c_int = 0xa02;
pub fn read_lsx(pid: libc::c_int) -> [[u64; 2]; 32] {
    let mut regs: [[u64; 2]; 32] = [[0; 2]; 32];
    read_regs(pid, &mut regs, NT_LOONGARCH_LSX);
    regs
}

const NT_LOONGARCH_LASX: libc::c_int = 0xa03;
pub fn read_lasx(pid: libc::c_int) -> [[u64; 4]; 32] {
    let mut regs: [[u64; 4]; 32] = [[0; 4]; 32];
    read_regs(pid, &mut regs, NT_LOONGARCH_LASX);
    regs
}

const NT_LOONGARCH_LBT: libc::c_int = 0xa04;
pub fn read_lbt(pid: libc::c_int) -> [u64; 5] {
    let mut regs: [u64; 5] = [0; 5];
    read_regs(pid, &mut regs, NT_LOONGARCH_LBT);
    regs
}

fn write_regs<T>(pid: libc::c_int, mut regs: T, core_note: libc::c_int) {
    let iovec: libc::iovec = libc::iovec {
        iov_base: &mut regs as *mut T as *mut libc::c_void,
        iov_len: size_of::<T>(),
    };
    assert_eq!(
        unsafe { libc::ptrace(libc::PTRACE_SETREGSET, pid, core_note, &iovec) },
        0
    );
}

pub fn write_gpr(pid: libc::c_int, regs: libc::user_regs_struct) {
    write_regs::<libc::user_regs_struct>(pid, regs, libc::NT_PRSTATUS);
}

pub fn write_fpr(pid: libc::c_int, regs: libc::user_fp_struct) {
    write_regs::<libc::user_fp_struct>(pid, regs, libc::NT_PRFPREG);
}

pub fn write_lsx(pid: libc::c_int, regs: [[u64; 2]; 32]) {
    write_regs::<[[u64; 2]; 32]>(pid, regs, NT_LOONGARCH_LSX);
}

pub fn write_lasx(pid: libc::c_int, regs: [[u64; 4]; 32]) {
    write_regs::<[[u64; 4]; 32]>(pid, regs, NT_LOONGARCH_LASX);
}
