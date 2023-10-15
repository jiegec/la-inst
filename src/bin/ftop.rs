use std::{arch::asm, thread::sleep, time::Duration};

fn main() {
    loop {
        unsafe {
            asm!("x86mttop 0");
        }
        let mut b: usize;
        unsafe {
            asm!("x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 0);

        sleep(Duration::from_secs_f32(0.1));

        let mut b: usize;
        unsafe {
            asm!("x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 0);

        unsafe {
            asm!("x86mttop 1");
        }

        let mut b: usize;
        unsafe {
            asm!("x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 1);

        sleep(Duration::from_secs_f32(0.1));

        let mut b: usize;
        unsafe {
            asm!("x86mftop {b}", b = out(reg) b);
        }
        assert_eq!(b, 1);
    }
}
