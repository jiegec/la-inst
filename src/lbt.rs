#[cfg(test)]
mod test {
    use std::arch::asm;

    #[test]
    fn test_scratch() {
        // copy registers to scratch
        let a = 1;
        let mut b;
        unsafe {
            asm!("movgr2scr $scr0, {a}
                  movscr2gr {b}, $scr0",
                a = in(reg) a,
                b = out(reg) b);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn test_jiscr() {
        // test jiscr
        let mut b: usize;
        unsafe {
            asm!("pcaddi {tmp}, 0
                  movgr2scr $scr0, {tmp}
                  jiscr0 20 # jump to li.d {b}, 2 below
                  li.d {b}, 1
                  b 8 # skip li.d {b}, 2 below
                  li.d {b}, 2", tmp = out(reg) _, b = out(reg) b);
        }
        assert_eq!(b, 2);
    }

    #[test]
    fn test_x86ftop() {
        // test x86 ftop
        let mut b: usize;
        unsafe {
            asm!("x86mttop 0
                  x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 0);

        unsafe {
            asm!("x86mttop 1
                  x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 1);

        unsafe {
            asm!("x86inctop
                  x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 2);

        unsafe {
            asm!("x86dectop
                  x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 1);

        unsafe {
            asm!("x86dectop
                  x86dectop
                  x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 7);

        unsafe {
            asm!("x86mttop 7
                  x86inctop
                  x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 0);
    }

    macro_rules! setx86j {
        ($eflags:expr, $condition:literal) => {
            {
                let mut res: usize;
                let eflags: usize = $eflags;
                unsafe {
                    asm!(concat!("x86mtflag {eflags}, 0x3f
                        setx86j {res}, " ,$condition),
                        eflags = in(reg) eflags,
                        res = out(reg) res);
                }
                res
            }
        };
    }

    #[test]
    fn test_setx86j() {
        // eflags https://en.wikipedia.org/wiki/FLAGS_register
        let cf = 0x01;
        let pf = 0x04;
        let zf = 0x40;
        let sf = 0x80;
        let of = 0x800;

        // 0: ja/jnbe: CF=0 && ZF=0
        assert_eq!(setx86j!(0, 0), 1);
        assert_eq!(setx86j!(cf, 0), 0);
        assert_eq!(setx86j!(pf, 0), 1);
        assert_eq!(setx86j!(zf, 0), 0);
        assert_eq!(setx86j!(sf, 0), 1);
        assert_eq!(setx86j!(of, 0), 1);
        assert_eq!(setx86j!(zf | cf, 0), 0);

        // 1: jnb/jze/jnc: CF=0
        assert_eq!(setx86j!(0, 1), 1);
        assert_eq!(setx86j!(cf, 1), 0);
        assert_eq!(setx86j!(pf, 1), 1);
        assert_eq!(setx86j!(zf, 1), 1);
        assert_eq!(setx86j!(sf, 1), 1);
        assert_eq!(setx86j!(of, 1), 1);

        // 2: jb/jnae/jc: CF=1
        assert_eq!(setx86j!(0, 2), 0);
        assert_eq!(setx86j!(cf, 2), 1);
        assert_eq!(setx86j!(pf, 2), 0);
        assert_eq!(setx86j!(zf, 2), 0);
        assert_eq!(setx86j!(sf, 2), 0);
        assert_eq!(setx86j!(of, 2), 0);

        // 3: jbe/jna: CF=1 || ZF=1
        assert_eq!(setx86j!(0, 3), 0);
        assert_eq!(setx86j!(cf, 3), 1);
        assert_eq!(setx86j!(pf, 3), 0);
        assert_eq!(setx86j!(zf, 3), 1);
        assert_eq!(setx86j!(sf, 3), 0);
        assert_eq!(setx86j!(of, 3), 0);
        assert_eq!(setx86j!(cf | zf, 3), 1);

        // 4: je/jz: ZF=1
        assert_eq!(setx86j!(0, 4), 0);
        assert_eq!(setx86j!(cf, 4), 0);
        assert_eq!(setx86j!(pf, 4), 0);
        assert_eq!(setx86j!(zf, 4), 1);
        assert_eq!(setx86j!(sf, 4), 0);
        assert_eq!(setx86j!(of, 4), 0);

        // 5: jne/jnz: ZF=0
        assert_eq!(setx86j!(0, 5), 1);
        assert_eq!(setx86j!(cf, 5), 1);
        assert_eq!(setx86j!(pf, 5), 1);
        assert_eq!(setx86j!(zf, 5), 0);
        assert_eq!(setx86j!(sf, 5), 1);
        assert_eq!(setx86j!(of, 5), 1);

        // 6: jg/jnle: ZF=0 && SF == OF
        assert_eq!(setx86j!(0, 6), 1);
        assert_eq!(setx86j!(cf, 6), 1);
        assert_eq!(setx86j!(pf, 6), 1);
        assert_eq!(setx86j!(zf, 6), 0);
        assert_eq!(setx86j!(sf, 6), 0);
        assert_eq!(setx86j!(of, 6), 0);
        assert_eq!(setx86j!(sf | of, 6), 1);

        // 7: jge/jnl: SF == OF
        assert_eq!(setx86j!(0, 7), 1);
        assert_eq!(setx86j!(cf, 7), 1);
        assert_eq!(setx86j!(pf, 7), 1);
        assert_eq!(setx86j!(zf, 7), 1);
        assert_eq!(setx86j!(sf, 7), 0);
        assert_eq!(setx86j!(of, 7), 0);
        assert_eq!(setx86j!(sf | of, 7), 1);

        // 8: jl/jnge: SF != OF
        assert_eq!(setx86j!(0, 8), 0);
        assert_eq!(setx86j!(cf, 8), 0);
        assert_eq!(setx86j!(pf, 8), 0);
        assert_eq!(setx86j!(zf, 8), 0);
        assert_eq!(setx86j!(sf, 8), 1);
        assert_eq!(setx86j!(of, 8), 1);
        assert_eq!(setx86j!(sf | of, 8), 0);

        // 9: jle/jng: ZF=1 || SF != OF
        assert_eq!(setx86j!(0, 9), 0);
        assert_eq!(setx86j!(cf, 9), 0);
        assert_eq!(setx86j!(pf, 9), 0);
        assert_eq!(setx86j!(zf, 9), 1);
        assert_eq!(setx86j!(sf, 9), 1);
        assert_eq!(setx86j!(of, 9), 1);
        assert_eq!(setx86j!(sf | of, 9), 0);

        // 10: js: SF=1
        assert_eq!(setx86j!(0, 10), 0);
        assert_eq!(setx86j!(cf, 10), 0);
        assert_eq!(setx86j!(pf, 10), 0);
        assert_eq!(setx86j!(zf, 10), 0);
        assert_eq!(setx86j!(sf, 10), 1);
        assert_eq!(setx86j!(of, 10), 0);

        // 11: jns: SF=0
        assert_eq!(setx86j!(0, 11), 1);
        assert_eq!(setx86j!(cf, 11), 1);
        assert_eq!(setx86j!(pf, 11), 1);
        assert_eq!(setx86j!(zf, 11), 1);
        assert_eq!(setx86j!(sf, 11), 0);
        assert_eq!(setx86j!(of, 11), 1);

        // 12: jo: OF=1
        assert_eq!(setx86j!(0, 12), 0);
        assert_eq!(setx86j!(cf, 12), 0);
        assert_eq!(setx86j!(pf, 12), 0);
        assert_eq!(setx86j!(zf, 12), 0);
        assert_eq!(setx86j!(sf, 12), 0);
        assert_eq!(setx86j!(of, 12), 1);

        // 13: jno: OF=0
        assert_eq!(setx86j!(0, 13), 1);
        assert_eq!(setx86j!(cf, 13), 1);
        assert_eq!(setx86j!(pf, 13), 1);
        assert_eq!(setx86j!(zf, 13), 1);
        assert_eq!(setx86j!(sf, 13), 1);
        assert_eq!(setx86j!(of, 13), 0);

        // 14: jp/jpe: PF=1
        assert_eq!(setx86j!(0, 14), 0);
        assert_eq!(setx86j!(cf, 14), 0);
        assert_eq!(setx86j!(pf, 14), 1);
        assert_eq!(setx86j!(zf, 14), 0);
        assert_eq!(setx86j!(sf, 14), 0);
        assert_eq!(setx86j!(of, 14), 0);

        // 15: jnp/jpo: PF=0
        assert_eq!(setx86j!(0, 15), 1);
        assert_eq!(setx86j!(cf, 15), 1);
        assert_eq!(setx86j!(pf, 15), 0);
        assert_eq!(setx86j!(zf, 15), 1);
        assert_eq!(setx86j!(sf, 15), 1);
        assert_eq!(setx86j!(of, 15), 1);
    }

    fn setx86loope(rj: usize, zf: bool) -> usize {
        let eflags = if zf { 0x40 } else { 0x0 };
        let mut res: usize;
        unsafe {
            asm!("x86mtflag {eflags}, 0x3f
                  setx86loope {res}, {rj}",
                  res = out(reg) res,
                  rj = in(reg) rj,
                  eflags = in(reg) eflags);
        }
        res
    }

    fn setx86loopne(rj: usize, zf: bool) -> usize {
        let eflags = if zf { 0x40 } else { 0x0 };
        let mut res: usize;
        unsafe {
            asm!("x86mtflag {eflags}, 0x3f
                  setx86loopne {res}, {rj}",
                  res = out(reg) res,
                  rj = in(reg) rj,
                  eflags = in(reg) eflags);
        }
        res
    }

    #[test]
    fn test_setx86loope() {
        // test x86 loope/loopne

        // loope: a != 0 && zf == 1
        assert_eq!(setx86loope(1, true), 1);
        assert_eq!(setx86loope(0, true), 0);
        assert_eq!(setx86loope(0, false), 0);
        assert_eq!(setx86loope(1, false), 0);

        // loopne: a != 0 && zf == 0
        assert_eq!(setx86loopne(1, true), 0);
        assert_eq!(setx86loopne(0, true), 0);
        assert_eq!(setx86loopne(0, false), 0);
        assert_eq!(setx86loopne(1, false), 1);
    }

    #[test]
    fn test_x86flags() {
        // test x86 mfflag/mtflag
        let eflags: u64 = 0xffffffffffffffff;
        let mut b: u64;
        unsafe {
            asm!("x86mtflag {eflags}, 0x3f
                  x86mfflag {b}, 0x3f",
                  b = out(reg) b,
                  eflags = in(reg) eflags);
        }
        // 0x8d5 = OF(0x800) |
        // SF(0x080) | ZF(0x040) | AF(0x010) |
        // PF(0x004) | CF(0x001)
        assert_eq!(b, 0x8d5);

        // different imm
        unsafe {
            asm!("x86mfflag {b}, 0x00",
                  b = out(reg) b);
        }
        assert_eq!(b, 0);

        // different imm
        unsafe {
            asm!("x86mfflag {b}, 0x01",
                  b = out(reg) b);
        }
        // CF
        assert_eq!(b, 0x1);

        unsafe {
            asm!("x86mfflag {b}, 0x02",
                  b = out(reg) b);
        }
        // PF
        assert_eq!(b, 0x4);

        unsafe {
            asm!("x86mfflag {b}, 0x03",
                  b = out(reg) b);
        }
        // CF | PF
        assert_eq!(b, 0x5);

        unsafe {
            asm!("x86mfflag {b}, 0x07",
                  b = out(reg) b);
        }
        // CF | PF | AF
        assert_eq!(b, 0x15);

        unsafe {
            asm!("x86mfflag {b}, 0x0f",
                  b = out(reg) b);
        }
        // CF | PF | AF | ZF
        assert_eq!(b, 0x55);

        unsafe {
            asm!("x86mfflag {b}, 0x30",
                  b = out(reg) b);
        }
        // SF | OF
        assert_eq!(b, 0x880);

        // partial set
        let eflags = 0;
        unsafe {
            asm!("x86mtflag {eflags}, 0x30
                  x86mfflag {b}, 0x3f",
                  b = out(reg) b,
                  eflags = in(reg) eflags);
        }
        // 0x55 = ZF(0x040) | AF(0x010) |
        // PF(0x004) | CF(0x001)
        assert_eq!(b, 0x55);

        let eflags = 0x800;
        unsafe {
            asm!("x86mtflag {eflags}, 0x30
                  x86mfflag {b}, 0x3f",
                  b = out(reg) b,
                  eflags = in(reg) eflags);
        }
        // 0x8d5 = OF(0x800) |
        // ZF(0x040) | AF(0x010) |
        // PF(0x004) | CF(0x001)
        assert_eq!(b, 0x855);
    }

    #[test]
    fn test_x86settag() {
        let mut b: usize = 0;
        unsafe {
            asm!("x86settag {b}, 0, 0",
                b = inout(reg) b);
        }
        assert_eq!(b, 1);

        let mut b: usize = 0;
        unsafe {
            asm!("x86settag {b}, 0, 1",
                b = inout(reg) b);
        }
        assert_eq!(b, 2);

        let mut b: usize = 0;
        unsafe {
            asm!("x86settag {b}, 0, 2",
                b = inout(reg) b);
        }
        assert_eq!(b, 4);

        let mut b: usize = 0;
        unsafe {
            asm!("x86settag {b}, 0, 3",
                b = inout(reg) b);
        }
        assert_eq!(b, 8);

        let mut b: usize = 0x4321;
        unsafe {
            asm!("x86settag {b}, 0, 3",
                b = inout(reg) b);
        }
        assert_eq!(b, 0x4329);

        let mut b: usize = 0x4321;
        unsafe {
            asm!("x86settag {b}, 0, 7",
                b = inout(reg) b);
        }
        assert_eq!(b, 0x43a1);

        let mut b: usize = 0x4321;
        unsafe {
            asm!("x86settag {b}, 1, 8",
                b = inout(reg) b);
        }
        assert_eq!(b, 0x4320);

        let mut b: usize = 0x4321;
        unsafe {
            asm!("x86settag {b}, 1, 8",
                b = inout(reg) b);
        }
        assert_eq!(b, 0x4320);
    }

    macro_rules! setarmj {
        ($nzcv:expr, $condition:literal) => {
            {
                let mut res: usize;
                let nzcv: usize = $nzcv;
                unsafe {
                    asm!(concat!("armmtflag {nzcv}, 0x39
                        setarmj {res}, " ,$condition),
                        nzcv = in(reg) nzcv,
                        res = out(reg) res);
                }
                res
            }
        };
    }

    #[test]
    fn test_setarmj() {
        // nzcv https://developer.arm.com/documentation/ddi0595/2021-06/AArch64-Registers/NZCV--Condition-Flags
        // A64 spec Table C1-1 Condition codes
        let n = 0x80000000;
        let z = 0x40000000;
        let c = 0x20000000;
        let v = 0x10000000;

        // 0: EQ: Z == 1
        assert_eq!(setarmj!(0, 0), 0);
        assert_eq!(setarmj!(n, 0), 0);
        assert_eq!(setarmj!(z, 0), 1);
        assert_eq!(setarmj!(c, 0), 0);
        assert_eq!(setarmj!(v, 0), 0);

        // 1: NE: Z == 0
        assert_eq!(setarmj!(0, 1), 1);
        assert_eq!(setarmj!(n, 1), 1);
        assert_eq!(setarmj!(z, 1), 0);
        assert_eq!(setarmj!(c, 1), 1);
        assert_eq!(setarmj!(v, 1), 1);

        // 2: CS or HS: C == 1
        assert_eq!(setarmj!(0, 2), 0);
        assert_eq!(setarmj!(n, 2), 0);
        assert_eq!(setarmj!(z, 2), 0);
        assert_eq!(setarmj!(c, 2), 1);
        assert_eq!(setarmj!(v, 2), 0);

        // 3: CC or LO: C == 0
        assert_eq!(setarmj!(0, 3), 1);
        assert_eq!(setarmj!(n, 3), 1);
        assert_eq!(setarmj!(z, 3), 1);
        assert_eq!(setarmj!(c, 3), 0);
        assert_eq!(setarmj!(v, 3), 1);

        // 4: MI: N == 1
        assert_eq!(setarmj!(0, 4), 0);
        assert_eq!(setarmj!(n, 4), 1);
        assert_eq!(setarmj!(z, 4), 0);
        assert_eq!(setarmj!(c, 4), 0);
        assert_eq!(setarmj!(v, 4), 0);

        // 5: PL: N == 0
        assert_eq!(setarmj!(0, 5), 1);
        assert_eq!(setarmj!(n, 5), 0);
        assert_eq!(setarmj!(z, 5), 1);
        assert_eq!(setarmj!(c, 5), 1);
        assert_eq!(setarmj!(v, 5), 1);

        // 6: VS: V == 1
        assert_eq!(setarmj!(0, 6), 0);
        assert_eq!(setarmj!(n, 6), 0);
        assert_eq!(setarmj!(z, 6), 0);
        assert_eq!(setarmj!(c, 6), 0);
        assert_eq!(setarmj!(v, 6), 1);

        // 7: VC: V == 0
        assert_eq!(setarmj!(0, 7), 1);
        assert_eq!(setarmj!(n, 7), 1);
        assert_eq!(setarmj!(z, 7), 1);
        assert_eq!(setarmj!(c, 7), 1);
        assert_eq!(setarmj!(v, 7), 0);

        // 8: HI: C == 1 && Z == 0
        assert_eq!(setarmj!(0, 8), 0);
        assert_eq!(setarmj!(n, 8), 0);
        assert_eq!(setarmj!(z, 8), 0);
        assert_eq!(setarmj!(c, 8), 1);
        assert_eq!(setarmj!(v, 8), 0);
        assert_eq!(setarmj!(c | z, 8), 0);

        // 9: LS: !(C == 1 && Z == 0)
        assert_eq!(setarmj!(0, 9), 1);
        assert_eq!(setarmj!(n, 9), 1);
        assert_eq!(setarmj!(z, 9), 1);
        assert_eq!(setarmj!(c, 9), 0);
        assert_eq!(setarmj!(v, 9), 1);
        assert_eq!(setarmj!(c | z, 9), 1);

        // 10: GE: N == V
        assert_eq!(setarmj!(0, 10), 1);
        assert_eq!(setarmj!(n, 10), 0);
        assert_eq!(setarmj!(z, 10), 1);
        assert_eq!(setarmj!(c, 10), 1);
        assert_eq!(setarmj!(v, 10), 0);
        assert_eq!(setarmj!(n | v, 10), 1);

        // 11: LT: N != V
        assert_eq!(setarmj!(0, 11), 0);
        assert_eq!(setarmj!(n, 11), 1);
        assert_eq!(setarmj!(z, 11), 0);
        assert_eq!(setarmj!(c, 11), 0);
        assert_eq!(setarmj!(v, 11), 1);
        assert_eq!(setarmj!(n | v, 11), 0);

        // 12: GT: Z == 0 && N == V
        assert_eq!(setarmj!(0, 12), 1);
        assert_eq!(setarmj!(n, 12), 0);
        assert_eq!(setarmj!(z, 12), 0);
        assert_eq!(setarmj!(c, 12), 1);
        assert_eq!(setarmj!(v, 12), 0);
        assert_eq!(setarmj!(n | v, 12), 1);
        assert_eq!(setarmj!(z | n | v, 12), 0);
        assert_eq!(setarmj!(z | n, 12), 0);

        // 13: LE: !(Z == 0 && N == V)
        assert_eq!(setarmj!(0, 13), 0);
        assert_eq!(setarmj!(n, 13), 1);
        assert_eq!(setarmj!(z, 13), 1);
        assert_eq!(setarmj!(c, 13), 0);
        assert_eq!(setarmj!(v, 13), 1);
        assert_eq!(setarmj!(n | v, 13), 0);
        assert_eq!(setarmj!(z | n | v, 13), 1);
        assert_eq!(setarmj!(z | n, 13), 1);

        // 14: AL: 1
        assert_eq!(setarmj!(0, 14), 1);
        assert_eq!(setarmj!(n, 14), 1);
        assert_eq!(setarmj!(z, 14), 1);
        assert_eq!(setarmj!(c, 14), 1);
        assert_eq!(setarmj!(v, 14), 1);

        // 15: NV: 1
        assert_eq!(setarmj!(0, 15), 1);
        assert_eq!(setarmj!(n, 15), 1);
        assert_eq!(setarmj!(z, 15), 1);
        assert_eq!(setarmj!(c, 15), 1);
        assert_eq!(setarmj!(v, 15), 1);
    }

    #[test]
    fn test_armflags() {
        // test arm mfflag/mtflag
        let eflags: u64 = 0xffffffffffffffff;
        let mut b: u64;
        unsafe {
            asm!("x86mtflag {eflags}, 0x3f
                  armmfflag {b}, 0x3f",
                  b = out(reg) b,
                  eflags = in(reg) eflags);
        }
        // sign extension
        // 0xf0000000 = N(0x800000000) |
        // Z(0x40000000) | C(0x20000000) |
        // V(0x10000000)
        assert_eq!(b, 0xfffffffff0000000);

        // different imm
        unsafe {
            asm!("armmfflag {b}, 0x00",
                  b = out(reg) b);
        }
        assert_eq!(b, 0);

        // different imm
        unsafe {
            asm!("armmfflag {b}, 0x01",
                  b = out(reg) b);
        }
        // CF = C
        assert_eq!(b, 0x20000000);

        unsafe {
            asm!("armmfflag {b}, 0x02",
                  b = out(reg) b);
        }
        // PF has no arm counterparts
        assert_eq!(b, 0x0);

        unsafe {
            asm!("armmfflag {b}, 0x03",
                  b = out(reg) b);
        }
        // CF | PF = C
        assert_eq!(b, 0x20000000);

        unsafe {
            asm!("armmfflag {b}, 0x07",
                  b = out(reg) b);
        }
        // CF | PF | AF = C
        assert_eq!(b, 0x20000000);

        unsafe {
            asm!("armmfflag {b}, 0x0f",
                  b = out(reg) b);
        }
        // CF | PF | AF | ZF = C | Z
        assert_eq!(b, 0x60000000);

        unsafe {
            asm!("armmfflag {b}, 0x30",
                  b = out(reg) b);
        }
        // SF | OF = N(0x80000000) | V(0x10000000)
        assert_eq!(b, 0xffffffff90000000);

        // partial set
        let eflags = 0;
        unsafe {
            asm!("x86mtflag {eflags}, 0x30
                  armmfflag {b}, 0x3f",
                  b = out(reg) b,
                  eflags = in(reg) eflags);
        }
        // 0x60000000 = Z(0x40000000) | C(0x20000000)
        assert_eq!(b, 0x60000000);

        let eflags = 0x800;
        unsafe {
            asm!("x86mtflag {eflags}, 0x30
                  armmfflag {b}, 0x3f",
                  b = out(reg) b,
                  eflags = in(reg) eflags);
        }
        // V(0x10000000) |
        // Z(0x40000000)
        // C(0x20000000)
        assert_eq!(b, 0x70000000);
    }
}
