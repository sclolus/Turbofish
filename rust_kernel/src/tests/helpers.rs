use crate::taskmaster::SysResult;

#[cfg(feature = "no-exit-qemu")]
pub fn exit_qemu(_exit_code: u32) -> SysResult<u32> {
    return Ok(0);
}

#[cfg(not(feature = "no-exit-qemu"))]
pub fn exit_qemu(exit_code: u32) -> SysResult<u32> {
    use io::{Io, Pio};
    let mut qemu_port = Pio::<u32>::new(0xf4);
    qemu_port.write(exit_code);
    unreachable!()
}
