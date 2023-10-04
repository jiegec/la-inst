# la-inst

Examine the effects of LoongArch instructions and discover undocumented ones.

Usage:

```shell
# discover undocumented instructions
$ cargo run --bin fuzz
# examine the behavior of instruction
$ cargo run --bin examine 00008020
Inst: 0x00008020, rd = 0, rj = 1, rk = 0
Binutils: Ok(Some("x86inc.b $r1"))
Ptrace: Register changed
LBT 1: OLD=0x00007fffd2cd0c00 NEW=0x0000000000000000
LBT 4: OLD=0x0000000000000000 NEW=0x0000000000000080
```