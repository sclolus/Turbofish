use crate::debug::POISON_SLAB;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(packed)]
pub struct Idtr {
    pub length: u16,
    pub idt_addr: *mut IdtGateEntry,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(packed)]
pub struct IdtGateEntry {
    pub offset_1: u16,
    pub selector: u16,
    pub _zero: u8,
    pub type_attr: u8,
    pub offset_2: u16,
}

pub struct InterruptTable<'a> {
    entries: &'a mut [IdtGateEntry]
}

extern "C" {
    fn asm_load_idtr(param: *mut Idtr) -> u32;
    fn asm_get_idtr() -> Idtr;
    fn asm_int(int: u32) -> ();
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn _load_idtr(mut idtr: Idtr) -> Idtr {
    unsafe {
        asm_load_idtr(&mut idtr as *mut Idtr);
        idtr
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn _asm_int(int: u32) -> () {
    unsafe {
        asm_int(int);
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn _asm_get_idtr() -> Idtr {
    unsafe {
        asm_get_idtr()
    }
}

impl Idtr {
    unsafe fn idt_gate_entries_slice_mut(&self) -> &mut [IdtGateEntry] {
        core::slice::from_raw_parts_mut(self.idt_addr, (self.length / 8) as usize)
    }

    unsafe fn idt_gate_entries_slice(&self) -> &[IdtGateEntry] {
        core::slice::from_raw_parts_mut(self.idt_addr, (self.length / 8) as usize)
    }

    pub unsafe fn get_interrupt_table(&self) -> InterruptTable {
        InterruptTable {
            entries: self.idt_gate_entries_slice_mut()
        }
    }
}


impl<'a> InterruptTable<'a> {
    pub fn enable_interrupt(&mut self, interrupt: usize) {
        self.entries[interrupt].type_attr |= 0x80;
    }

    pub fn disable_interrupt(&mut self, interrupt: usize) {
        self.entries[interrupt].type_attr &= !0x80;
    }

    pub fn disable_all_interrupts(&mut self) {
        for interrupt in 0..self.entries.len() {
            self.disable_interrupt(interrupt);
        }
    }

    pub fn set_interrupt_entry(&mut self, interrupt: usize, entry: &IdtGateEntry) {
        self.entries[interrupt] = *entry;
    }
}
