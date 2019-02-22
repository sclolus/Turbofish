/// Ivybridge+ RDRAND feature.
/// rdrand set the carry flag to 1 if the random is well done, else loop while it works
pub fn rdrand() -> u32 {
    let result: u32;

    unsafe {
        asm!("
            1:
            rdrand %eax
            jnc 1b" : "={eax}"(result) :::);
    }
    result
}
