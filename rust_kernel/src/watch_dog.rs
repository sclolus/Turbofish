use crate::interrupts::idt::{IdtGateEntry, InterruptTable};
use crate::memory::tools::sections::{__end_rodata, __end_text, __start_rodata, __start_text};
#[allow(deprecated)]
use core::hash::{Hash, Hasher, SipHasher};

type IdtBios = [[u16; 2]; 256];

struct WatchDog {
    pub checksum_text: u64,
    pub checksum_rodata: u64,
    pub idt: [IdtGateEntry; InterruptTable::DEFAULT_IDT_SIZE as usize],
    pub idt_size: usize,
    pub idt_bios: IdtBios,
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
        let curr_idt = InterruptTable::current_interrupt_table();
        let idt_bios = *(0x0 as *const IdtBios);
        match WATCH_DOG {
            None => {
                let mut idt: [IdtGateEntry; InterruptTable::DEFAULT_IDT_SIZE as usize] = core::mem::zeroed();

                for (d, s) in idt.iter_mut().zip(curr_idt.iter()) {
                    *d = *s;
                }
                WATCH_DOG = Some(WatchDog { checksum_text, checksum_rodata, idt, idt_size: curr_idt.len(), idt_bios });
            }
            Some(WatchDog {
                checksum_text: old_checksum_text,
                checksum_rodata: old_checksum_rodata,
                idt,
                idt_size,
                idt_bios: old_idt_bios,
            }) => {
                assert_eq!(idt_size, curr_idt.len(), "corruption of idt, idt size has changed");
                let old_idt = core::slice::from_raw_parts(idt.as_ptr(), idt_size);
                for (i, (o, n)) in old_idt.iter().zip(curr_idt.iter()).enumerate() {
                    assert_eq!(o, n, "\ncorruption of idt at offset '{}'", i);
                }
                for (i, (o, n)) in old_idt_bios.iter().zip(idt_bios.iter()).enumerate() {
                    assert_eq!(o, n, "\ncorruption of idtbios at offset '{}'", i);
                }
                if checksum_rodata != old_checksum_rodata {
                    panic!("rodata corruption detected");
                }
                if checksum_text != old_checksum_text {
                    panic!("text corruption detected");
                }
            }
        }
        // to test the watch dog
        //*(symbol_addr!(__start_text) as *mut u8) = 42;
    }
}
