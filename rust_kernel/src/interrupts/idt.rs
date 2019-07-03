//! See [IDT](https://wiki.osdev.org/IDT)
//! and [Interrupts](https://wiki.osdev.org/Interrupts)
mod cpu_exceptions_isr;
use cpu_exceptions_isr::*;

use bit_field::BitField;
use core::ffi::c_void;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::slice::SliceIndex;

extern "C" {
    fn _default_isr();
}

pub type InterruptHandler = extern "C" fn() -> !;

/// The GateType is the type of Gate for the IdtGateEntry.
/// Gate Type 	Possible IDT gate types :
///              0b0101	0x5	5	80386 32 bit task gate
///              0b0110	0x6	6	80286 16-bit interrupt gate
///              0b0111	0x7	7	80286 16-bit trap gate
///              0b1110	0xE	14	80386 32-bit interrupt gate
///              0b1111	0xF	15	80386 32-bit trap gate
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GateType {
    TaskGate32 = 0x5,
    InterruptGate16 = 0x6,
    TrapGate16 = 0x7,
    InterruptGate32 = 0xE,
    TrapGate32 = 0xF,
    InvalidGateType,
}

/// We might from to implement TryFrom instead of From.
impl From<u8> for GateType {
    fn from(value: u8) -> Self {
        match value {
            0x5 => TaskGate32,
            0x6 => InterruptGate16,
            0x7 => TrapGate16,
            0xE => InterruptGate32,
            0xF => TaskGate32,
            _ => InvalidGateType,
        }
    }
}

use GateType::*;

/// The IdtGateEntry is a type of descriptor that populates the Interrupt Descriptor Table.
/// When trying to fire an interrupt of interrupt number `intn`, the CPU will look inside the IDT at index `indn`,
/// and if possible depending on the different flags in the entry, will call the registered handler in the entry.
/// The handler's address is contained in two parts inside the fields `offset_1` and `offset_2`.
/// The handler's address is actually an offset inside the Segment, contained inside the GDT/LDT, indexed by the Selector `Selector`.
/// If the CPU has to ignore the entry, (because it's present bitflag is not set or in other cases) the CPU will throw a Double Fault.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C, packed)]
pub struct IdtGateEntry {
    /// offset bits 0..15. the low part of the address
    pub offset_1: u16,

    /// a code segment selector in GDT or LDT
    pub selector: u16,

    /// unused, set to 0
    pub _zero: u8,

    /// The type attr is layout in this way.
    ///
    /// ```None
    ///  7                             0  
    /// +---+---+---+---+---+---+---+---+  
    /// | P |  DPL  | S |    GateType   |  
    /// +---+---+---+---+---+---+---+---+  
    /// - P:          Present	Set to 0 for unused interrupts.  
    /// - DPL:        Descriptor Privilege Level	Gate call protection.
    ///               Specifies which privilege Level the calling Descriptor minimum
    ///               should have.
    ///               So hardware and CPU interrupts can be protected from
    ///               being called out of userspace.  
    /// - S:          Storage Segment	Set to 0 for interrupt and trap gates  
    /// - Gate Type:  Possible IDT gate types :
    ///               0b0101	0x5	5	80386 32 bit task gate
    ///               0b0110	0x6	6	80286 16-bit interrupt gate
    ///               0b0111	0x7	7	80286 16-bit trap gate
    ///               0b1110	0xE	14	80386 32-bit interrupt gate
    ///               0b1111	0xF	15	80386 32-bit trap gate  
    /// ```
    /// type and attributes,
    pub type_attr: u8,

    /// offset bits 16..31. The high part of the address.
    pub offset_2: u16,
}

impl IdtGateEntry {
    /// Returns a minimal IdtGateEntry, which is just a zeroed out entry.
    fn minimal() -> Self {
        unsafe { core::mem::zeroed() }
    }

    /// Returns a new IdtGateEntry, the entry is set as present and all other fields are zeroed out.
    pub fn new() -> Self {
        let mut new = Self::minimal();

        new.set_present(true);
        new
    }

    /// Sets the present flag of the IdtGateEntry.
    /// If this is set, then the Gate is used by the cpu when the corresponding interrupt is triggered.
    /// If this is not, then the Gate is unused.
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.type_attr.set_bit(7, present);
        self
    }

    /// Gets the value of the present flag of the IdtGateEntry.
    /// If this is set, then the Gate is used by the cpu when the corresponding interrupt is triggered.
    /// If this is not, then the Gate is unused.
    pub fn present(&self) -> bool {
        self.type_attr.get_bit(7)
    }

    /// Sets the value of the storage segment flag of the entry.
    pub fn set_storage_segment(&mut self, storage: bool) -> &mut Self {
        self.type_attr.set_bit(4, storage);
        self
    }

    /// Gets the value of the storage segment flag of the entry.
    pub fn storage_segment(&self) -> bool {
        self.type_attr.get_bit(4)
    }

    /// This sets the GateType of the entry.
    pub fn set_gate_type(&mut self, gate_type: GateType) -> &mut Self {
        self.type_attr.set_bits(0..4, gate_type as u8);
        self
    }

    /// Gets the GateType of the entry.
    pub fn gate_type(&self) -> GateType {
        self.type_attr.get_bits(0..4).into()
    }

    /// This sets the DPL (Descriptor Privilege Level) of the entry.
    /// If the privilege level of the current user is not atleast of DPL (0 being the highest), a General Protection Fault will be thrown.
    pub fn set_privilege_level(&mut self, dpl: u8) -> &mut Self {
        self.type_attr.set_bits(5..7, dpl);
        self
    }

    /// Gets the DPL (Descriptor Privilege Level) of the entry.
    /// If the privilege level of the current user is not atleast of DPL (0 being the highest), a General Protection Fault will be thrown.
    pub fn get_privilege_level(&self) -> u8 {
        self.type_attr.get_bits(5..7)
    }

    /// Sets the Selector of the entry. This is the selector in the GDT (or LDT) containing the handler for the interrupt.
    pub fn set_selector(&mut self, selector: u16) -> &mut Self {
        self.selector = selector;
        self
    }

    /// Sets the handler of the entry.
    /// The lower half of the handler address is stored in member `offset_1`, while the higher half is stored in member `offset_2`.
    pub fn set_handler(&mut self, handler: u32) -> &mut Self {
        self.offset_1 = handler as u16;
        self.offset_2 = ((handler as usize) >> 16) as u16;
        self
    }

    /// Gets the handler of the entry.
    /// The lower half of the handler address is stored in member `offset_1`, while the higher half is stored in member `offset_2`.
    pub fn handler(&self) -> u32 {
        ((self.offset_2 as u32) << 16) | (self.offset_1 as u32)
    }
}

/// This is the Interrupt Descriptor Table Register representation. It contains the `length in bytes - 1` and the address of the IDT.
/// It may be generated and then loaded in the actual register IDTR, which tells the CPU to start using the IDT at `idt_addr` for Interrupt Gates lookup.
/// It can be consumed in order to obtain an InterruptTable struct which is the interface to modify IdtGateEntries.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
#[repr(packed)]
pub struct Idtr {
    /// The `length in bytes - 1` of the IDT.
    pub length: u16,

    /// The address of the IDT.
    pub idt_addr: *mut IdtGateEntry,
}

/// Returns the default values for the Idtr.
impl Default for Idtr {
    fn default() -> Self {
        Idtr {
            length: InterruptTable::DEFAULT_IDT_SIZE - 1,
            idt_addr: InterruptTable::DEFAULT_IDT_ADDR,
        }
    }
}

impl Idtr {
    /// Consumes the Idtr, returning the corresponding InterruptTable.
    unsafe fn interrupt_table<'a>(self) -> InterruptTable<'a> {
        InterruptTable {
            entries: core::slice::from_raw_parts_mut(
                self.idt_addr,
                ((self.length + 1) / 8) as usize,
            ),
        }
    }

    /// Returns the current Interrupt Descriptor Table Register
    #[no_mangle]
    #[inline(never)]
    unsafe extern "C" fn get_idtr() -> Idtr {
        // Temporary struct Idtr to be filled by the asm routine
        let mut idtr = Idtr {
            length: 0,
            idt_addr: 1 as *mut _,
        };

        asm!("sidt $0" : "=*m"(&mut idtr as *mut _) :: "memory" : "volatile");
        idtr
    }

    /// Loads the contents of `idtr` into the Interrupt Descriptor Table Register.
    #[no_mangle]
    #[inline(never)]
    unsafe extern "C" fn load_idtr(&self) {
        asm!("lidt ($0)" :: "r" (self as *const _) : "memory" : "volatile");
    }

    /// Loads the Idtr structure in the actual register IDTR.
    /// It then creates an InterruptTable struct, consuming the Idtr struct (to prevent aliasing of the memory zone used by the InterruptTable),
    /// loads the default configuration of the InterruptTable,
    /// and finally, returns it.
    pub unsafe fn init_idt<'a>(self) -> InterruptTable<'a> {
        without_interrupts!({
            self.load_idtr();

            let mut idt = self.interrupt_table();

            idt.init_default_exceptions();
            idt.init_cpu_exceptions();
            INITIALIZED = true;
            idt
        })
    }
}

/// This is the representation of the IDT (Interrupt Descriptor Table).
/// It consists of a slice of IdtGateEntries.
/// Its lifetime is basically a fake unbounded lifetime, this issue can't be resolved.
/// The InterruptTable is the interface to modify the entries inside the IDT.
#[derive(Debug)]
pub struct InterruptTable<'a> {
    entries: &'a mut [IdtGateEntry],
}

/// The InterruptTable implements Index which enables us to use the syntax: `idt[index]`,
/// instead of `idt.entries[index]` in an immutable context.
/// This generic implementation also enables us to use the syntax idt[n..m] or any other Range slice indexing.
///
/// # Panics
/// Panics if `index` is outside of the InterruptTable, that is, if index >= InterruptTable.entries.len()
impl<'a, T> Index<T> for InterruptTable<'a>
where
    T: SliceIndex<[IdtGateEntry]>,
{
    type Output = T::Output;

    #[inline]
    fn index(&self, idx: T) -> &Self::Output {
        idx.index(self.entries)
    }
}

/// The InterruptTable implements IndexMut which enables us to use the syntax: `idt[index] = SomeIdtGateEntry`
/// instead of `idt.entries[index] = SomeIdtGateEntry` in a mutable context.
/// This generic implementation also enables us to use the syntax idt[n..m] or any other Range slice indexing.
///
/// # Panics
/// Panics if `index` is outside of the InterruptTable, that is, if index >= InterruptTable.entries.len()
impl<'a, T> IndexMut<T> for InterruptTable<'a>
where
    T: SliceIndex<[IdtGateEntry]>,
{
    #[inline]
    fn index_mut(&mut self, idx: T) -> &mut Self::Output {
        idx.index_mut(self.entries)
    }
}

/// The InterruptTable implements Deref, which makes it a Smart Pointer.
/// The main purpose of this is to enable the coersion of a &InterruptTable in a slice of IdtGateEntries: `&\[IdtGateEntries\]`,
/// which basically means that all the immutable methods of &\[IdtGateEntries\] (the slice methods) are available for the InterruptTable.
impl Deref for InterruptTable<'_> {
    type Target = [IdtGateEntry];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.entries
    }
}

/// The InterruptTable implements DerefMut, which makes it a Smart Pointer.
/// The main purpose of this is to enable the coersion of a &mut InterruptTable in a mutable slice of IdtGateEntries: `&mut \[IdtGateEntries\]`,
/// which basically means that all the mutable methods of &mut \[IdtGateEntries\] (the slice methods) are available for the InterruptTable.
impl DerefMut for InterruptTable<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entries
    }
}

static mut INITIALIZED: bool = false;

impl InterruptTable<'_> {
    /// This is the default size (in bytes) of the IDT, and also the maximum size of the IDT on x86.
    /// As an IdtGateEntry has a size of 8 bytes, there are 256 entries in the table.
    pub const DEFAULT_IDT_SIZE: u16 = 256 * 8;

    /// This is the default address of the IDT.
    const DEFAULT_IDT_ADDR: *mut IdtGateEntry = 0x1000 as *mut _;

    /// The list of the default exception handlers.
    /// They are loaded by the `init_cpu_exceptions` method.
    const CPU_EXCEPTIONS: [(unsafe extern "C" fn() -> !, GateType); 32] = [
        (_isr_divide_by_zero, InterruptGate32),
        (_isr_debug, TrapGate32),
        (_isr_non_maskable_interrupt, InterruptGate32),
        (_isr_breakpoint, TrapGate32),
        (_isr_overflow, TrapGate32),
        (_isr_bound_range_exceeded, InterruptGate32),
        (_isr_invalid_opcode, InterruptGate32),
        (_isr_no_device, InterruptGate32),
        (_isr_double_fault, InterruptGate32),
        (_isr_fpu_seg_overrun, InterruptGate32),
        (_isr_invalid_tss, InterruptGate32),
        (_isr_seg_no_present, InterruptGate32),
        (_isr_stack_seg_fault, InterruptGate32),
        (_isr_general_protect_fault, InterruptGate32),
        (_isr_page_fault, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (_isr_fpu_floating_point_exep, InterruptGate32),
        (_isr_alignment_check, InterruptGate32),
        (_isr_machine_check, InterruptGate32),
        (_isr_simd_fpu_fp_exception, InterruptGate32),
        (_isr_virtualize_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (_isr_security_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
    ];

    /// Set the CPYU exceptions vectors on the first 32 entries.
    /// # Panics
    /// Panics if the interruptions are not disabled when this is called, that is, if interrupts::get_interrupts_state() == true.
    unsafe fn init_cpu_exceptions(&mut self) {
        assert!(super::get_interrupts_state() == false); // Should be turned in a debug_assert! eventually.

        let mut gate_entry = *IdtGateEntry::new()
            .set_storage_segment(false)
            .set_privilege_level(0)
            .set_selector(1 << 3)
            .set_gate_type(InterruptGate32);

        for (index, &(exception, gate_type)) in Self::CPU_EXCEPTIONS.iter().enumerate() {
            gate_entry
                .set_handler(exception as *const c_void as u32)
                .set_gate_type(gate_type);

            self[index] = gate_entry;
        }
    }

    /// Set the basic defauly exception handler in all the IDT table
    /// # Panics
    /// Panics if the interruptions are not disabled when this is called, that is, if interrupts::get_interrupts_state() == true.
    unsafe fn init_default_exceptions(&mut self) {
        assert!(super::get_interrupts_state() == false); // Should be turned in a debug_assert! eventually.

        let mut gate_entry = *IdtGateEntry::new()
            .set_storage_segment(false)
            .set_privilege_level(0)
            .set_selector(1 << 3)
            .set_gate_type(InterruptGate32);

        for entry in self.iter_mut() {
            *entry = *gate_entry.set_handler(_default_isr as *const c_void as u32);
        }
    }

    /// Gets the current InterruptTable as specified by the current IDTR.
    /// This method is basically just a shorthand.
    pub unsafe fn current_interrupt_table<'a>() -> Option<InterruptTable<'a>> {
        match INITIALIZED {
            false => None,
            true => Some(Idtr::get_idtr().interrupt_table()),
        }
    }
}
