use std::arch::asm;

fn main() {
    /* let x: u64;

    unsafe {
        asm!("mov {}, 5",out(reg) x);
    }

    assert_eq!(x, 5); */

    /* let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
        );
    }

    assert_eq!(o, 8); */

    /* let mut x: u64 = 3;
    unsafe {
        asm!("add {0}, 5", inout(reg) x);
    }
    assert_eq!(x, 8); */

    /* let x: u64 = 3;
    let y: u64;
    unsafe {
        asm!("add {0}, 5",inout(reg)x=>y);
    }
    assert_eq!(y, 8); */

    /* let mut a: u64 = 4;
    let b: u64 = 4;
    let c: u64 = 4;
    unsafe {
        asm!(
            "add {0}, {1}",
            "add {0}, {2}",
            inout(reg) a,
            in(reg) b,
            in(reg) c,
        );
    }
    assert_eq!(a, 12); */

    /* let mut a: u64 = 4;
    let b: u64 = 4;
    unsafe {
        asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
    }
    assert_eq!(a, 8); */

    let cmd = 0xd1;
    unsafe {
        asm!("out 0x64, eax", in("eax") cmd);
    }
}
