use anyhow;
use std::io::Write;
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
