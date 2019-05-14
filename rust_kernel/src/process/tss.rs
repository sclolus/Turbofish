#[allow(unused)]
#[derive(Default, Debug)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[repr(C)]
pub struct Tss {
    /*0x00*/ link: u16, _reserved1: u16,
    /*0x04*/ esp0: u32,
    /*0x08*/ ss0: u16, _reserved2: u16,
    /*0x0C*/ esp1: u32,
    /*0x10*/ ss1: u16, _reserved3: u16,
    /*0x14*/ esp2: u32,
    /*0x18*/ ss2: u16, _reserved4: u16,
    /*0x1C*/ cr3: u32,
    /*0x20*/ eip: u32,
    /*0x24*/ eflags: u32,
    /*0x28*/ eax: u32,
    /*0x2C*/ ecx: u32,
    /*0x30*/ edx: u32,
    /*0x34*/ ebx: u32,
    /*0x38*/ esp: u32,
    /*0x3C*/ ebp: u32,
    /*0x40*/ esi: u32,
    /*0x44*/ edi: u32,
    /*0x48*/ es: u16, _reserved5: u16,
    /*0x4C*/ cs: u16, _reserved6: u16,
    /*0x50*/ ss: u16, _reserved7: u16,
    /*0x54*/ ds: u16, _reserved8: u16,
    /*0x58*/ fs: u16, _reserved9: u16,
    /*0x5C*/ gs: u16, _reserved10: u16,
    /*0x60*/ ldtr: u16, _reserved11: u16,
    /*0x64*/ debug_flag: u16, io_map: u16,
}

#[allow(unused)]
impl Tss {
    const TSS_MEMORY_ADDRESS: u32 = 0x1800;

    /// Init a new TSS descriptor at TSS_MEMORY_ADDRESS (must be unique)
    pub unsafe fn init(esp: u32, ss: u16) -> *mut Self {
        let tss: *mut Tss = Self::TSS_MEMORY_ADDRESS as *mut Tss;
        *tss = Tss { ss0: ss, esp0: esp, ..Default::default() };
        tss
    }

    /// Reassign stack value for the TSS descriptor
    pub fn reset(&mut self, esp: u32, ss: u16) {
        self.ss0 = ss;
        self.esp0 = esp;
    }

    /// Display the current TSS segment
    pub fn display() {
        let tss: *mut Tss = Self::TSS_MEMORY_ADDRESS as *mut Tss;
        unsafe {
            println!("{:#?}", *tss);
        }
    }
}
