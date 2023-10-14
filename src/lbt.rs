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
        let mut b: u64;
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
        let mut b: u64;
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
}
