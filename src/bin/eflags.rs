use std::{arch::asm, thread::sleep, time::Duration};

fn main() {
    loop {
        let eflags: usize = 0;
        unsafe {
            asm!("x86mtflag {eflags}, 0x3f", eflags = in(reg) eflags);
        }

        let mut b: usize;
        unsafe {
            asm!("x86mfflag {b}, 0x3f", b = out(reg) b);
        }
        assert_eq!(b, 0);

        sleep(Duration::from_secs_f32(0.1));

        let mut b: usize;
        unsafe {
            asm!("x86mfflag {b}, 0x3f", b = out(reg) b);
        }
        assert_eq!(b, 0);

        let eflags: usize = 0x1;
        unsafe {
            asm!("x86mtflag {eflags}, 0x3f", eflags = in(reg) eflags);
        }

        let mut b: usize;
        unsafe {
            asm!("x86mfflag {b}, 0x3f", b = out(reg) b);
        }
        assert_eq!(b, 1);

        sleep(Duration::from_secs_f32(0.1));

        let mut b: usize;
        unsafe {
            asm!("x86mfflag {b}, 0x3f", b = out(reg) b);
        }
        assert_eq!(b, 1);
    }
}
