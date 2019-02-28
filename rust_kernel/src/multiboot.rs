use crate::mm::NbrPages;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(packed)]
#[derive(Default)]
pub struct MultibootInfo {
    /*0        |*/ pub flags: u32, // required
    /*         +*/
    /*4        |*/ pub mem_lower: u32, // present if flags[0] is set
    /*8        |*/ pub mem_upper: u32, // present if flags[0] is set
    /*         +*/
    /*12       |*/ pub boot_device: u32, // present if flags[1] is set
    /*         +*/
    /*16       |*/ pub cmdline: u32, // present if flags[2] is set
    /*         +*/
    /*20       |*/ pub mods_count: u32, // present if flags[3] is set
    /*24       |*/ pub mods_addr: u32, // present if flags[3] is set
    /*         +*/
    /* 28 - 40 |*/ pub syms: [u32; 3], // present if flags[4] or flags[5] is set
    /* 44      |*/ pub mmap_length: u32, // present if flags[6] is set
    /* 48      |*/ pub mmap_addr: u32, // present if flags[6] is set
    /*         +*/
    /* 52      |*/ pub drives_length: u32, // present if flags[7] is set
    /* 56      |*/ pub drives_addr: u32, // present if flags[7] is set
    /*         +*/
    /* 60      |*/ pub config_table: u32, // present if flags[8] is set
    /*         +*/
    /* 64      |*/ pub boot_loader_name: u32, // present if flags[9] is set
    /*         +*/
    /* 68      |*/ pub apm_table: u32, // present if flags[10] is set
    /*         +*/
    /* 72      |*/ pub vbe_control_info: u32, // present if flags[11] is set
    /* 76      |*/ pub vbe_mode_info: u32,
    /* 80      |*/ pub vbe_mode: u16,
    /* 82      |*/ pub vbe_interface_seg: u16,
    /* 84      |*/ pub vbe_interface_off: u16,
    /* 86      |*/ pub vbe_interface_len: u16,
    /*         +*/
    /* 88      |*/ pub framebuffer_addr: u64, // present if flags[12] is set
    /* 96      |*/ pub framebuffer_pitch: u32,
    /* 100     |*/ pub framebuffer_width: u32,
    /* 104     |*/ pub framebuffer_height: u32,
    /* 108     |*/ pub framebuffer_bpp: u8,
    /* 109     |*/ pub framebuffer_typ: u8,
    /* 110-115 |*/ pub color_info: [u8; 5],
}

pub static mut MULTIBOOT_INFO: Option<MultibootInfo> = None;

pub fn save_multiboot_info(multiboot_info: *const MultibootInfo) {
    unsafe {
        MULTIBOOT_INFO = Some(*multiboot_info);
    }
}

impl MultibootInfo {
    // add 1Mo because mem upper start after 1Mo
    pub fn get_memory_amount_nb_pages(&self) -> NbrPages {
        NbrPages((self.mem_upper as usize + 1024) / 4)
    }
    pub fn get_system_memory_amount(&self) -> usize {
        (self.mem_upper as usize + 1024) * 1024
    }

    pub fn get_system_starting_addr(&self) -> usize {
        (self.mem_lower as usize + 1024) * 1024
    }
}
