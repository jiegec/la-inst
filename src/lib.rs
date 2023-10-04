use anyhow;
use opcode::OPCODES;
use ptrace::*;
use rand::Rng;
use std::arch::asm;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

pub mod opcode;
pub mod ptrace;

pub fn inst_legal_binutils(inst: u32) -> bool {
    for (value, mask) in OPCODES {
        if inst & *mask == *value {
            return true;
        }
    }
    false
}

// Discovered undocumented opcodes
pub const KNOWN_OPCODES: &[(u32, u32)] = &[
    (0x01147400, 0xfffffc00), // frecip.s
    (0x01147800, 0xfffffc00), // frecip.d
    (0x01148400, 0xfffffc00), // frsqrt.s
    (0x01148800, 0xfffffc00), // frsqrt.d
    (0x0114c000, 0xfffffc00), // movgr2fcsr
    (0x0114c800, 0xfffffc00), // movfcsr2gr
    (0x38570000, 0xffff8000), // unknown memory
    (0x38578000, 0xfffff000), // unknown memory
    (0x38580000, 0xfffd8000), // ld.b
    (0x38588000, 0xfffd8000), // ld.h
    (0x38590000, 0xfffd8000), // ld.w
    (0x38598000, 0xfffd8000), // ld.d
    (0x385c0000, 0xfffd8000), // amswap.b
    (0x385c8000, 0xfffd8000), // amswap.h
    (0x385d0000, 0xfffd8000), // amadd.b
    (0x385d8000, 0xfffd8000), // amadd.h
    (0x71448000, 0xffff8000), // unknown lsx
    (0x71450000, 0xffff8000), // unknown lsx
    (0x729b8000, 0xffff8000), // lsx vindex
    (0x729d2400, 0xfffffc00), // lsx vfrsqrt.s
    (0x729d2800, 0xfffffc00), // lsx vfrsqrt.d
    (0x75448000, 0xffff8000), // unknown lasx
    (0x75450000, 0xffff8000), // unknown lasx
    (0x769b8000, 0xffff8000), // lasx vindex
    (0x769d2400, 0xfffffc00), // lasx vfrsqrt.s
    (0x769d2800, 0xfffffc00), // lasx vfrsqrt.d
];


pub fn inst_discovered(inst: u32) -> bool {
    for (value, mask) in KNOWN_OPCODES {
        if inst & *mask == *value {
            return true;
        }
    }
    false
}

/* Return decoded inst if legal */
pub fn inst_decode_binutils(inst: u32) -> anyhow::Result<Option<String>> {
    // check if instruction is legal via binutils
    let mut file = NamedTempFile::new()?;
    file.write(&inst.to_le_bytes())?;
    let path = file.into_temp_path();
    let command = Command::new("objdump")
        .args([
            "-b",
            "binary",
            "-m",
            "Loongarch64",
            "-M",
            "numeric,no-aliases",
            "-D",
            path.to_str().unwrap(),
        ])
        .output()?;
    let stdout = String::from_utf8(command.stdout)?;
    if let Some(last_line) = stdout.lines().last() {
        let mut decoded = String::new();
        for part in last_line.split("\t").skip(2) {
            if part == ".word" {
                return Ok(None);
            }
            if decoded.len() > 0 {
                decoded += " ";
            }
            decoded += part.trim();
        }
        Ok(Some(decoded))
    } else {
        Err(anyhow::anyhow!("unexpected objdump output"))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct RegisterSet {
    pub gpr: [u64; 32],
    pub lasx: [[u64; 4]; 32],
    pub lbt: [u64; 5],
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct RegisterInfo {
    pub old: RegisterSet,
    pub new: RegisterSet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProbeResult {
    IllegalInstruction,
    SegmentationFault,
    BusError,
    RegisterUnchaged,
    RegisterChanged(RegisterInfo),
}

/* Set register values instead of randomization */
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegisterPreset {
    GeneralRegister(usize, u64),
    LASXRegister(usize, [u64; 4]),
}

/* Check if instruction is legal via ptrace */
pub fn inst_legal_ptrace(inst: u32, presets: &[RegisterPreset]) -> anyhow::Result<ProbeResult> {
    // setup instruction page
    let page_size = 16384;
    let stack_page = unsafe {
        libc::mmap(
            0 as *mut libc::c_void,
            page_size,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
            0,
            0,
        )
    };
    let inst_page = unsafe {
        libc::mmap(
            0 as *mut libc::c_void,
            page_size,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
            0,
            0,
        )
    };
    let bytes = inst.to_le_bytes();
    unsafe {
        inst_page.copy_from(&bytes as *const u8 as *const libc::c_void, 4);
    }

    // initialize stack page
    for i in 0..page_size {
        unsafe {
            *(stack_page as *mut u8).add(i) = (i + 1) as u8;
        }
    }

    // fork a child process
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // in child process
        unsafe {
            // activate LASX extension for kernel to initalize context
            // so we can access LASX registers later, instead of seeing filled 0xff
            // asm!("xvadd.b $xr0, $xr0, $xr0");
            asm!(".word 0x740a0000");

            // ask parent to ptrace me
            libc::ptrace(libc::PTRACE_TRACEME, 0, 0, 0);
            libc::raise(libc::SIGSTOP);
            libc::exit(0);
        }
    }

    // in parent
    // wait for child SIGSTOP
    let mut status: libc::c_int = 0;
    unsafe {
        libc::waitpid(pid, &mut status, 0);
    }

    // read register set
    let mut regs = read_gpr(pid);
    let mut lasx_regs = read_lasx(pid);
    let lbt_regs = read_lbt(pid);

    // randomize all regs
    let mut rng = rand::thread_rng();
    for i in 0..32 {
        regs.regs[i] = rng.gen();
        lasx_regs[i] = rng.gen();
    }

    // process presets
    for preset in presets {
        match preset {
            RegisterPreset::GeneralRegister(index, value) => regs.regs[*index] = *value,
            RegisterPreset::LASXRegister(index, value) => lasx_regs[*index] = *value,
        }
    }

    // r0 is hardwared to zero
    regs.regs[0] = 0;

    // set pc and sp(r3)
    regs.csr_era = inst_page as u64;
    regs.regs[3] = stack_page as u64;

    // sync regs and single step
    write_gpr(pid, regs);
    write_lasx(pid, lasx_regs);
    unsafe {
        libc::ptrace(libc::PTRACE_SINGLESTEP, pid, 0, 0);
    }

    // wait for child signal
    unsafe {
        libc::waitpid(pid, &mut status, 0);
    }
    assert!(libc::WIFSTOPPED(status));

    // check signal
    let result = if libc::WSTOPSIG(status) == libc::SIGILL {
        // illegal instruction
        ProbeResult::IllegalInstruction
    } else if libc::WSTOPSIG(status) == libc::SIGSEGV {
        // segmentation fault
        ProbeResult::SegmentationFault
    } else if libc::WSTOPSIG(status) == libc::SIGBUS {
        // bus error
        ProbeResult::BusError
    } else if libc::WSTOPSIG(status) == libc::SIGTRAP {
        // normal trap

        // check if register changed
        let regs_new = read_gpr(pid);
        let lasx_regs_new = read_lasx(pid);
        let lbt_regs_new = read_lbt(pid);
        if regs.regs == regs_new.regs && lasx_regs == lasx_regs_new && lbt_regs == lbt_regs_new {
            ProbeResult::RegisterUnchaged
        } else {
            // collect regs
            let mut info = RegisterInfo::default();
            info.old.gpr = regs.regs;
            info.old.lasx = lasx_regs;
            info.old.lbt = lbt_regs;
            info.new.gpr = regs_new.regs;
            info.new.lasx = lasx_regs_new;
            info.new.lbt = lbt_regs_new;

            ProbeResult::RegisterChanged(info)
        }
    } else {
        unimplemented!("unknown signal {:?}", libc::WSTOPSIG(status));
    };

    // cleanup child process and memory
    unsafe {
        libc::kill(pid, libc::SIGKILL);
        libc::waitpid(pid, &mut status, 0);
        libc::munmap(stack_page, page_size);
        libc::munmap(inst_page, page_size);
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_legal() {
        assert_eq!(
            inst_decode_binutils(0x02ffc063).unwrap(),
            Some("addi.d $r3, $r3, -16".to_string())
        );
        assert_eq!(
            inst_decode_binutils(0x72eff00c).unwrap(),
            Some("vpickve2gr.d $r12, $vr0, 0x0".to_string())
        );
    }

    #[test]
    fn test_illegal() {
        assert_eq!(inst_decode_binutils(0x0).unwrap(), None);
    }
}
