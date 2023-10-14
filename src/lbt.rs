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
}
