//! impl write on the [Serial Ports](https://wiki.osdev.org/Serial_ports)
use bitflags::bitflags;
use core::fmt;
use io::{Io, Pio};

bitflags! {
    /// Line status flags
    struct LineStsFlags: u8 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}

pub struct Uart16550 {
    data: Pio<u8>,
    int_en: Pio<u8>,
    fifo_ctrl: Pio<u8>,
    line_ctrl: Pio<u8>,
    modem_ctrl: Pio<u8>,
    line_sts: Pio<u8>,
}

impl Uart16550 {
    pub const fn new(base: u16) -> Self {
        Self {
            data: Pio::new(base),
            int_en: Pio::new(base + 1),
            fifo_ctrl: Pio::new(base + 2),
            line_ctrl: Pio::new(base + 3),
            modem_ctrl: Pio::new(base + 4),
            line_sts: Pio::new(base + 5),
        }
    }

    pub fn init(&mut self) {
        self.int_en.write(0x00); // disable all interrupts
        self.line_ctrl.write(0x80); // enable DLAB (set baud rate divisor
        self.data.write(0x03); // Set divisor to 3 (lo byte) 38400 baud
        self.int_en.write(0x00); //                (hi byte)
        self.line_ctrl.write(0x03); // 8 bits, no parity, one stop bit
        self.fifo_ctrl.write(0xC7);
        self.modem_ctrl.write(0x0B); // Enable FIFO, clear them, with 14-byte threshold
        self.int_en.write(0x01); // IRQs enabled, RTS/DSR set
    }

    fn line_sts(&self) -> LineStsFlags {
        LineStsFlags::from_bits_truncate(self.line_sts.read())
    }

    pub fn send(&mut self, byte: u8) {
        while !self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {}
        self.data.write(byte);
    }
}

impl fmt::Write for Uart16550 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}

pub static mut UART_16550: Uart16550 = Uart16550::new(0x3F8);

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        match format_args!($($arg)*) {
            a => {
                #[allow(unused_unsafe)]
                core::fmt::write(unsafe {&mut $crate::uart_16550::UART_16550}, a).unwrap();
            }
        }
    }
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
}
