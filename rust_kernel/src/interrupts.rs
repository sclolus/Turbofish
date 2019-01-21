use crate::debug::POISON_SLAB;

#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub offset_1: u16,  // offset bits 0..15. the low part of the address
    pub selector: u16,  // a code segment selector in GDT or LDT
    pub _zero: u8,      // unused, set to 0


    // The type attr is layout in this way.
    //   7                           0
    // +---+---+---+---+---+---+---+---+
    // | P |  DPL  | S |    GateType   |
    // +---+---+---+---+---+---+---+---+



    // P        	Present	Set to 0 for unused interrupts.
    // DPL          Descriptor Privilege Level	Gate call protection.
    //              Specifies which privilege Level the calling Descriptor minimum
    //              should have.
    //              So hardware and CPU interrupts can be protected from
    //              being called out of userspace.
    // S            Storage Segment	Set to 0 for interrupt and trap gates
    // Gate Type 	Possible IDT gate types :
    //              0b0101	0x5	5	80386 32 bit task gate
    //              0b0110	0x6	6	80286 16-bit interrupt gate
    //              0b0111	0x7	7	80286 16-bit trap gate
    //              0b1110	0xE	14	80386 32-bit interrupt gate
    //              0b1111	0xF	15	80386 32-bit trap gate

    pub type_attr: u8,  // type and attributes, see below
    pub offset_2: u16,  // offset bits 16..31
}

pub struct InterruptTable<'a> {
    entries: &'a mut [IdtGateEntry]
}

extern "C" {
    fn asm_load_idtr(param: *const Idtr);
    fn asm_get_idtr(to_fill: *mut Idtr);
    fn asm_int(int: u32) -> ();

    pub    fn generic_asm_isr_wrapper();
}

#[no_mangle]
#[inline(never)]
// Load the idtr passed in parameter into the Interrupt descriptor Table Register
pub extern "C" fn _load_idtr(idtr: Idtr) -> Idtr {
    unsafe {
        asm_load_idtr(&idtr as *const Idtr);
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
pub extern "C" fn _get_idtr() -> Idtr {
    unsafe {
        // Temporary struct Idtr to be filled by the asm routine
        let mut idtr = Idtr {
            length: 0,
            idt_addr: 1 as *mut _,
        };

        asm_get_idtr(&mut idtr as *mut _);
        idtr
    }
}


impl Idtr {
    const DEFAULT_IDTR_LENGTH: u16 = 1024;
    const DEFAULT_IDTR_ADDR: *mut IdtGateEntry = 0x400 as *mut _;

    // Current default idtr, the address is 0x400, just above the idt bios
    // and just below the current GDT
    const DEFAULT_IDTR: Idtr = Idtr {
        length: Idtr::DEFAULT_IDTR_LENGTH,
        idt_addr: Idtr::DEFAULT_IDTR_ADDR,
    };

    // Loads the default idtr
    pub  unsafe fn load_default_idtr() -> Idtr {
        _load_idtr(Self::DEFAULT_IDTR)
    }

    // Primitives to get a slice of the IDT entries.
    unsafe fn idt_gate_entries_slice_mut(&self) -> &mut [IdtGateEntry] {
        core::slice::from_raw_parts_mut(self.idt_addr, (self.length / 8) as usize)
    }

    unsafe fn idt_gate_entries_slice(&self) -> &[IdtGateEntry] {
        core::slice::from_raw_parts_mut(self.idt_addr, (self.length / 8) as usize)
    }

    // Construct the Interrupt Table.
    pub unsafe fn get_interrupt_table(&self) -> InterruptTable {
        InterruptTable {
            entries: self.idt_gate_entries_slice_mut()
        }
    }
}


impl<'a> InterruptTable<'a> {

    // Set the P flag in type_attr to 1
    // WARNING: This is not an sli call
    // The interrupt is merely enabled if sli() was called
    pub fn enable_interrupt(&mut self, interrupt: usize) {
        self.entries[interrupt].type_attr |= 0x80;
    }

    // Set the P flag in type_attr to 1
    pub fn disable_interrupt(&mut self, interrupt: usize) {
        self.entries[interrupt].type_attr &= !0x80;
    }

    // Sets the P flag in type_attr to 0 for all the Gate Entries
    // Warning: This is not an cli call
    // The interrupts can still be fired.
    pub fn disable_all_interrupts(&mut self) {
        for interrupt in 0..self.entries.len() {
            self.disable_interrupt(interrupt);
        }
    }

    // Sets a perticular Gate entry to a specific value.
    pub fn set_interrupt_entry(&mut self, interrupt: usize, entry: &IdtGateEntry) {
        self.entries[interrupt] = *entry;
    }
}
