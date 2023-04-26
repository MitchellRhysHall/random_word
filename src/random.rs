use core::arch::asm;

#[inline]
pub(crate) fn generate_u64() -> u64 {
    // First attempt hardware rng
    let mut result: u64;
    let mut success: u8;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "rdrand {0}",
            "setc {1}",
            out(reg) result,
            out(reg_byte) success,
            options(nostack),
        );
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!(
            "mrs {0}, RNDR",
            "mov {1}, #1",
            out(reg) result,
            out(reg_byte) success,
            options(nostack),
        );
    }

    if success == 1 {
        result
    } else {
        panic!("Only x86_64 and aarch64 is supported for random number generation!")
    }
}

#[cfg(test)]
#[test]
fn rng_test() {
    let mut prev1 = 0;
    (1..999999).for_each(|_| {
        let new1 = generate_u64();
        assert_ne!(prev1, new1);
        prev1 = new1;
    })
}
