use super::SysResult;

extern "C" {
    fn _get_esp() -> u32;
}

/// Do a stack overflow on the kernel stack
#[allow(unconditional_recursion)]
pub unsafe fn sys_stack_overflow(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> SysResult<u32> {
    unpreemptible_context!({
        println!(
            "Stack overflow syscall on the fly: v = {:?}, esp: {:#X?}",
            a + (b + c + d + e + f) * 0,
            _get_esp()
        );
    });

    Ok(sys_stack_overflow(a + 1, b + 1, c + 1, d + 1, e + 1, f + 1).unwrap())
}
