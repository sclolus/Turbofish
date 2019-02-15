use crate::io::{Io, Pio};

pub fn exit_qemu(exit_code: u32) {
    let mut qemu_port = Pio::<u32>::new(0xf4);
    qemu_port.write(exit_code);
}
