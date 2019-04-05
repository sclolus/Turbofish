use crate::interrupts::idt::{IdtGateEntry, InterruptTable};
use crate::memory::tools::sections::{__end_rodata, __end_text, __start_rodata, __start_text};
#[allow(deprecated)]
use core::hash::{Hash, Hasher, SipHasher};

type IvtBios = [[u16; 2]; 256];

struct WatchDog {
    pub checksum_text: u64,
    pub checksum_rodata: u64,
    pub idt: [IdtGateEntry; InterruptTable::DEFAULT_IDT_SIZE as usize],
    pub idt_size: usize,
    pub ivt_bios: IvtBios,
}

static mut WATCH_DOG: Option<WatchDog> = None;

pub fn watch_dog() {
    let hash_section = |start: usize, end: usize| -> u64 {
        #[allow(deprecated)]
        let mut hasher = SipHasher::new();
        let section = unsafe { core::slice::from_raw_parts(start as *const u8, end - start) };
        section.hash(&mut hasher);
        hasher.finish()
    };

    let checksum_text = hash_section(symbol_addr!(__start_text), symbol_addr!(__end_text));
    let checksum_rodata = hash_section(symbol_addr!(__start_rodata), symbol_addr!(__end_rodata));
    unsafe {
        let curr_idt = InterruptTable::current_interrupt_table().unwrap();
        let ivt_bios = *(0x0 as *const IvtBios);
        match &mut WATCH_DOG {
            None => {
                let mut idt: [IdtGateEntry; InterruptTable::DEFAULT_IDT_SIZE as usize] = core::mem::zeroed();

                for (d, s) in idt.iter_mut().zip(curr_idt.iter()) {
                    *d = *s;
                }
                WATCH_DOG = Some(WatchDog { checksum_text, checksum_rodata, idt, idt_size: curr_idt.len(), ivt_bios });
            }
            Some(watchdog) => {
                assert_eq!(watchdog.idt_size, curr_idt.len(), "corruption of idt, idt size has changed");

                // test if modifications are occured in IDT
                for (i, (o, n)) in watchdog.idt.iter().zip(curr_idt.iter()).enumerate() {
                    if o != n {
                        log::warn!("Watchdog is worried about IDT at offset {}", i);
                    }
                }
                // refresh watchdog IDT
                for (d, s) in watchdog.idt.iter_mut().zip(curr_idt.iter()) {
                    *d = *s;
                }
                // test if modifications are occured in IVT BIOS
                for (i, (o, n)) in watchdog.ivt_bios.iter().zip(ivt_bios.iter()).enumerate() {
                    if o != n {
                        log::warn!("Watchdog is worried about bios IVT at offset {}", i);
                    }
                }
                // refresh watchdog IVT BIOS
                watchdog.ivt_bios = ivt_bios;

                if checksum_rodata != watchdog.checksum_rodata {
                    panic!("rodata corruption detected");
                }
                if checksum_text != watchdog.checksum_text {
                    panic!("text corruption detected");
                }
            }
        }
        // to test the watch dog on text segment
        //*(symbol_addr!(__start_text) as *mut u8) = 42;
        // to test watch dog on BIOS IVT
        //watchdog.ivt_bios[42] = [0x42, 0x42];
        // to test watch dog on IDT
        //let p: *mut u8 = 0x1018 as *mut u8;
        //*p = 42 as u8;
    }
}
