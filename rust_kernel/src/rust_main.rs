use crate::vga::*;

#[derive(Debug)]
#[repr(C)]
#[repr(packed)]
pub struct MultibootInfos {
    /*0        |*/ pub flags: u32,              //|    (required)
    /*         +*/
    /*4        |*/ pub mem_lower: u32,          //|    (present if flags[0] is set)
    /*8        |*/ pub mem_upper: u32,          //|    (present if flags[0] is set)
    /*         +*/
    /*12       |*/ pub boot_device: u32,        //|    (present if flags[1] is set)
    /*         +*/
    /*16       |*/ pub cmdline: u32,            //|    (present if flags[2] is set)
    /*         +*/
    /*20       |*/ pub mods_count: u32,         //|    (present if flags[3] is set)
    /*24       |*/ pub mods_addr: u32,          //|    (present if flags[3] is set)
    /*         + */
    /* 28 - 40 | */pub syms: [u32; 3],          //|    (present if flags[4] or
    /*         | */                   //|                flags[5] is set)
    /* 44      | */pub mmap_length: u32,        //|    (present if flags[6] is set)
    /* 48      | */pub mmap_addr: u32,          //|    (present if flags[6] is set)
    /*         +-*/ 
    /* 52      | */pub drives_length: u32,      //|    (present if flags[7] is set)
    /* 56      | */pub drives_addr: u32,        //|    (present if flags[7] is set)
    /*         +-*/
    /* 60      | */pub config_table: u32,       //|    (present if flags[8] is set)
    /*         +-*/
    /* 64      | */pub boot_loader_name: u32,   //|    (present if flags[9] is set)
    /*         +-*/
    /* 68      | */pub apm_table: u32,          //|    (present if flags[10] is set)
    /*         +-*/
    /* 72      | */pub vbe_control_info: u32,   //|    (present if flags[11] is set)
    /* 76      | */pub vbe_mode_info: u32,      
    /* 80      | */pub vbe_mode: u16,           
    /* 82      | */pub vbe_interface_seg: u16,  
    /* 84      | */pub vbe_interface_off: u16,  
    /* 86      | */pub vbe_interface_len: u16,  
    /*         +-*/
    /* 88      | */pub framebuffer_addr: u64,   //|    (present if flags[12] is set)
    /* 96      | */pub framebuffer_pitch: u32,  
    /* 100     | */pub framebuffer_width: u32,  
    /* 104     | */pub framebuffer_height: u32, 
    /* 108     | */pub framebuffer_bpp: u8,    
    /* 109     | */pub framebuffer_typ: u8,
    /* 110-115 | */pub color_info: [u8; 5],
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_infos: *const MultibootInfos) {
    clear_screen();
    println!("multiboot_infos {:?}", multiboot_infos);
    println!("multiboot_infos {:?}", (*multiboot_infos));
    /*
    set_text_color("yellow").unwrap();
    for _x in 0..2 {
        println!("test\nPrintln");
        println!("vga term {:#?}", VGA_TERM);
        println!();
        print!("E");
        println!("RTV");
        println!("RTV");
    }
    match set_text_color("alacrityKikooColor") {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    match set_text_color("brown") {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    match set_cursor_position(40, 24) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    match set_cursor_position(42, 42) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    set_cursor_position(42, 42).unwrap();
    */
    loop {}
}
