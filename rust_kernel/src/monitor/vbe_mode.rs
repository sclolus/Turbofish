use crate::monitor::core_monitor::*;
use core::fmt::Write;

const TEMPORARY_PTR_LOCATION: *mut u8 = 0x2000 as *mut u8;

#[derive(Copy, Clone)]
#[repr(C)]
#[repr(packed)]
struct VesaGlobalInfo {
    /*0        |*/ pub vesa_signature: [char; 4],
    /*4        |*/ pub vesa_version: u16,
    /*6        |*/ pub oem_name: [char; 4],
    /*10       |*/ pub capability_flag: u32,
    /*14       |*/ pub list_supported_mode_offset: u16,
    /*16       |*/ pub list_supported_mode_segment: u16,
    /*18       |*/ pub memory_ammount: u16,
    /*20       |*/ pub vbe_2_field: [u8; 236],
    /*256      |*/
}

const MAX_NB_VESA_MODE: usize = 126;
#[derive(Copy, Clone)]
#[repr(C)]
#[repr(packed)]
struct VesaGraphicModeList {
    /*0        |*/ pub mode: [u16; MAX_NB_VESA_MODE],
    /*252      |*/ pub nb_mode: u32,
    /*256      |*/
}

#[derive(Copy, Clone)]
#[repr(C)]
#[repr(packed)]
struct VesaModeInfo {
    /*0        |*/ pub attributes: u16,    //| (deprecated, only bit 7 should be of interest to you, and it indicates the mode supports a linear frame buffer)
    /*2        |*/ pub window_a: u8,       //| (deprecated)
    /*3        |*/ pub window_b: u8,       //| (deprecated)
    /*4        |*/ pub granularity: u16,   //| (deprecated used while calculating bank numbers)
    /*6        |*/ pub window_size: u16,
    /*8        |*/ pub segment_a: u16,
    /*10       |*/ pub segment_b: u16,
    /*12       |*/ pub win_func_ptr: u32,  //| (deprecated; used to switch banks from protected mode without returning to real mode)
    /*16       |*/ pub pitch: u16,         //| (number of bytes per horizontal line)
    /*18       |*/ pub width: u16,         //| (width in pixels)
    /*20       |*/ pub height: u16,        //| (height in pixels)
    /*22       |*/ pub w_char: u8,         //| (unused...)
    /*23       |*/ pub y_char: u8,         //| (...)
    /*24       |*/ pub planes: u8,
    /*25       |*/ pub bpp: u8,            //| (bits per pixel in this mode)
    /*26       |*/ pub banks: u8,          //| (deprecated total number of banks in this mode)
    /*27       |*/ pub memory_model: u8,
    /*28       |*/ pub bank_size: u8,      //| (deprecated; size of a bank, almost always 64 KB but may be 16 KB...)
    /*29       |*/ pub image_pages: u8,
    /*30       |*/ pub reserved0: u8,
    /*         +*/
    /*31       |*/ pub red_mask: u8,
    /*32       |*/ pub red_position: u8,
    /*33       |*/ pub green_mask: u8,
    /*34       |*/ pub green_position: u8,
    /*35       |*/ pub blue_mask: u8,
    /*36       |*/ pub blue_position: u8,
    /*37       |*/ pub reserved_mask: u8,
    /*38       |*/ pub reserved_position: u8,
    /*39       |*/ pub direct_color_attributes: u8,
    /*         +*/
    /*40       |*/ pub framebuffer: *mut u8,     //| (physical address of the linear frame buffer write here to draw to the screen)
    /*44       |*/ pub off_screen_mem_off: u32,
    /*48       |*/ pub off_screen_mem_size: u16, //| (size of memory in the frame buffer but not being displayed on the screen)
    /*50       |*/ pub reserved1: [u8; 206],
    /*256      |*/
}

static mut VESA_GLOBAL_INFO: Option<VesaGlobalInfo> = None;

static mut VESA_MODE_INFO: Option<VesaModeInfo> = None;

#[derive(Debug)]
pub struct VbeMode {
    memory_location: *mut u8,
    mode:u8,
    width:usize,
    height:usize,
    bpp:u8,
    x:usize,
    y:usize,
    char_height:u8,
    char_width:u8,
    nb_lines:usize,
    nb_colomns:usize,
}

pub static mut SVGA_VBE: VbeMode =
    VbeMode {memory_location: 0 as *mut u8, mode: 0, width: 0, height: 0, bpp: 0,
             x: 0, y: 0, char_width: 0, char_height: 0, nb_lines: 0, nb_colomns: 0};

impl IoScreen for VbeMode {
    fn set_graphic_mode(&mut self, mode:u8) -> Result {
        self.mode = mode;
        Ok(())
    }
    fn putchar(&mut self, _c:char) -> Result {
        Ok(())
    }
    fn scroll_screen(&mut self) -> Result {
        Ok(())
    }
    fn clear_screen(&mut self) -> Result {
        use crate::support::memset;
        unsafe {
            memset(self.memory_location, 0, self.bpp as usize * self.width * self.height);
        }
        Ok(())
    }
    fn set_text_color(&mut self, _color:TextColor) -> Result {
        Ok(())
    }
    fn set_cursor_position(&mut self, _x:usize, _y:usize) -> Result {
        Ok(())
    }
}

impl Write for VbeMode {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            match *c as char {
                '\n' => {
                    self.x = 0;
                    self.y = self.y + 1;
                    if self.y == self.height {
                        self.scroll_screen().unwrap();
                    }
                }
                _ => {
                    self.putchar(*c as char).unwrap();
                    self.x = self.x + 1;
                    if self.x == self.width {
                        self.x = 0;
                        self.y = self.y + 1;
                        if self.y == self.height {
                            self.scroll_screen().unwrap();;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
