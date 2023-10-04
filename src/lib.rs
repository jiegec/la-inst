use anyhow;
use std::io::Write;
use std::mem::size_of;
use std::process::Command;
use tempfile::NamedTempFile;

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

fn read_regs(pid: libc::c_int) -> libc::user_regs_struct {
    let mut regs: libc::user_regs_struct = libc::user_regs_struct {
        regs: [0; 32],
        orig_a0: 0,
        csr_era: 0,
        csr_badv: 0,
        reserved: [0; 10],
    };
    let iovec: libc::iovec = libc::iovec {
        iov_base: &mut regs as *mut libc::user_regs_struct as *mut libc::c_void,
        iov_len: size_of::<libc::user_regs_struct>(),
    };
    unsafe {
        libc::ptrace(libc::PTRACE_GETREGSET, pid, libc::NT_PRSTATUS, &iovec);
    }

    // r0 is always zero
    regs.regs[0] = 0;

    regs
}

fn write_regs(pid: libc::c_int, mut regs: libc::user_regs_struct) -> libc::user_regs_struct {
    let iovec: libc::iovec = libc::iovec {
        iov_base: &mut regs as *mut libc::user_regs_struct as *mut libc::c_void,
        iov_len: size_of::<libc::user_regs_struct>(),
    };
    unsafe {
        libc::ptrace(libc::PTRACE_SETREGSET, pid, libc::NT_PRSTATUS, &iovec);
    }
    regs
}

#[derive(Clone, Debug)]
pub enum ProbeResult {
    IllegalInstruction,
    SegmentationFault,
    RegisterUnchaged,
    // list of register changed: (index, old, new)
    RegisterChanged(Vec<(usize, u64, u64)>),
}

/* Check if instruction is legal via ptrace */
pub fn inst_legal_ptrace(inst: u32) -> anyhow::Result<ProbeResult> {
    // setup instruction page
    let page_size = 16384;
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

    // fork a child process
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // in child process
        // ask parent to ptrace me
        unsafe {
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
    let mut regs = read_regs(pid);

    // clear all regs
    regs.regs.fill(0);

    // set pc and single step
    regs.csr_era = inst_page as u64;
    write_regs(pid, regs);
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
    } else if libc::WSTOPSIG(status) == libc::SIGTRAP {
        // normal trap

        // check if register changed
        let regs_new = read_regs(pid);
        if regs.regs == regs_new.regs {
            ProbeResult::RegisterUnchaged
        } else {
            // collect changed regs
            let mut changed = vec![];
            for i in 0..32 {
                if regs.regs[i] != regs_new.regs[i] {
                    changed.push((i, regs.regs[i], regs_new.regs[i]));
                }
            }
            ProbeResult::RegisterChanged(changed)
        }
    } else {
        unimplemented!("unknown signal {:?}", libc::WSTOPSIG(status));
    };

    // cleanup child process
    unsafe {
        libc::kill(pid, libc::SIGKILL);
        libc::waitpid(pid, &mut status, 0);
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
