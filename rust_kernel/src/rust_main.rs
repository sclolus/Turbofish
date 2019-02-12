use crate::debug;
use crate::interrupts;
use crate::interrupts::pit::*;
use crate::interrupts::{pic_8259, Idtr, PIC_8259};
use crate::monitor::bmp_loader::*;
use crate::monitor::*;
use crate::multiboot::{save_multiboot_info, MultibootInfo, MULTIBOOT_INFO};
use crate::timer::Rtc;

extern "C" {
    static _asterix_bmp_start: BmpImage;
    static _wanggle_bmp_start: BmpImage;
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo) -> u32 {
    save_multiboot_info(multiboot_info);
    println!("multiboot_infos {:#?}", MULTIBOOT_INFO);
    println!("base memory: {:?} {:?}", MULTIBOOT_INFO.unwrap().mem_lower, MULTIBOOT_INFO.unwrap().mem_upper);

    unsafe {
        interrupts::disable();

        Idtr::default().init_idt();
        PIC_8259.init();
        PIC_8259.disable_all_irqs();
        PIC_8259.enable_irq(pic_8259::Irq::KeyboardController); // enable only the keyboard.

        interrupts::enable();
    }

    unsafe {
        SCREEN_MONAD.switch_graphic_mode(Some(0x118)).unwrap();
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
        SCREEN_MONAD.clear_screen();
        SCREEN_MONAD
            .draw_graphic_buffer(|buffer: *mut u8, width: usize, height: usize, bpp: usize| {
                draw_image(&_asterix_bmp_start, buffer, width, height, bpp)
            })
            .unwrap();

        PIT0.configure(OperatingMode::RateGenerator);
        PIT0.start_at_frequency(18.0).unwrap();
        PIC_8259.enable_irq(pic_8259::Irq::SystemTimer);
    }
    debug::bench_start();
    for _i in 0..3 {
        println!("
use crate::ffi::c_char;
use crate::monitor::core_monitor::IoResult;
use crate::monitor::core_monitor::{{Cursor, Drawer, TextColor}};
use crate::registers::{{BaseRegisters, _real_mode_op}};
use core::result::Result;

#[macro_export]
macro_rules! impl_raw_data_debug {{
    ($e:ty) => {{
        impl core::fmt::Debug for $e {{
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {{
                write!(f, \"raw data at address {{:?}}: {{}} bytes reserved\", &self.0 as *const u8, self.0.len())
            }}
        }}
    }};
}}

#[macro_export]
macro_rules! define_raw_data {{
    ($name:ident, $size_in_bytes:expr) => {{
        #[derive(Copy, Clone)]
        #[repr(C)]
        #[repr(packed)]
        pub struct $name(pub [u8; $size_in_bytes]);
        impl_raw_data_debug!($name);
    }};
}}

const TEMPORARY_PTR_LOCATION: *mut u8 = 0x2000 as *mut u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[repr(packed)]
pub struct VbeInfo {{
    /*0  */ pub vbe_signature: [c_char; 4], //db 'VESA' ; VBE Signature
    /*4  */ pub vbe_version: u16, //dw 0300h ; vbe version
    /*6  */ pub oem_string_offset: u32, //dd ? ; vbe_far_offset to oem string
    /*10 */ pub capabilities: u32, //db 4 dup (?) ; capabilities of graphics controller
    /*14 */ pub video_mode_offset: u32, //dd ? ; vbe_far_offset to video_mode_list
    /*18 */ pub total_memory: u16, //dw ? ; number of 64kb memory blocks added for vbe 2.0+
    /*20 */ pub oem_software_rev: u16, //dw ? ; vbe implementation software revision
    /*22 */ pub oem_vendor_name_offset: u32, //dd ? ; vbe_far_offset to vendor name string
    /*26 */ pub oem_product_name_offset: u32, //dd ? ; vbe_far_offset to product name string
    /*30 */ pub oem_product_rev_offset: u32, //dd ? ; vbe_far_offset to product revision string
    /*34 */ pub reserved: VbeInfoReserved, //db 222 dup (?) ; reserved for vbe implementation scratch area
    /*256*/ pub oem_data: VbeInfoOemData, //db 256 dup ; data area for oem strings
}}

define_raw_data!(VbeInfoReserved, 222);
define_raw_data!(VbeInfoOemData, 256);

impl VbeInfo {{
    /// only way to initialize VbeInfo safely transform all the pointers within the struct by their offsets
    unsafe fn new(ptr: *const Self) -> Self {{
        Self {{ video_mode_offset: (*ptr).video_mode_offset - ptr as u32, ..*ptr }}
    }}
    /// calculate the mode ptr using the address of self and the offset
    fn get_video_mode_ptr(&self) -> *const u16 {{
        unsafe {{ (self as *const Self as *const u8).add(self.video_mode_offset as usize) as *const u16 }}
    }}
    /// return the number of modes available
    /// The VideoModePtr is a VbeFarPtr that points to a list of mode numbers for all display modes
    /// supported by the VBE implementation. Each mode number occupies one word (16 bits). The list
    /// of mode numbers is terminated by a -1 (0FFFFh). The mode numbers in this list represent all of
    /// the potentially supported modes by the display controller.
    fn nb_mode(&self) -> usize {{
        let mut i = 0;
        let video_mode_ptr = self.get_video_mode_ptr();
        unsafe {{
            while *((video_mode_ptr).offset(i as isize)) != 0xFFFF {{
                i += 1;
                // 111 is the maximum number of modes because reserved is 222 bytes
                if i >= 111 {{
                    return i;
                }}
            }}
        }}
        i
    }}
    /// return an iterator on available modes
    pub fn iter_modes(&self) -> core::slice::Iter<u16> {{
        unsafe {{ core::slice::from_raw_parts(self.get_video_mode_ptr(), self.nb_mode()).iter() }}
    }}
    /// return the best resolution mode available which is in 3 bytes color if any.
    pub fn find_best_resolution_mode(&self) -> (u16, ModeInfo) {{
        self.iter_modes()
            .map(|m| (*m, query_mode_info(*m).unwrap()))
            .max_by(|(_, a), (_, b)| {{
                if a.bits_per_pixel != b.bits_per_pixel {{
                    a.bits_per_pixel.cmp(&b.bits_per_pixel) // more bits for pixel is better
                }} else {{
                    (a.x_resolution + a.y_resolution).cmp(&(b.x_resolution + b.y_resolution))
                }}
            }})
            .unwrap()
    }}
}}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[repr(packed)]
pub struct ModeInfo {{
    /// Mandatory information for all VBE revisions
    mode_attributes: u16, // dw ? ; mode attributes
    win_a_attributes: u8,     // db ? ; window A attributes
    win_b_attributes: u8,     // db ? ; window B attributes
    win_granularity: u16,     // dw ? ; window granularity
    win_size: u16,            // dw ? ; window size
    win_a_segment: u16,       // dw ? ; window A start segment
    win_b_segment: u16,       // dw ? ; window B start segment
    win_func_ptr: u32,        // dd ? ; real mode pointer to window function
    bytes_per_scan_line: u16, // dw ? ; bytes per scan line
    /// Mandatory information for VBE 1.2 and above
    x_resolution: u16, // dw ? ; horizontal resolution in pixels or characters 3
    y_resolution: u16,        // dw ? ; vertical resolution in pixels or characters
    x_char_size: u8,          // db ? ; character cell width in pixels
    y_char_size: u8,          // db ? ; character cell height in pixels
    number_of_planes: u8,     // db ? ; number of memory planes
    bits_per_pixel: u8,       // db ? ; bits per pixel
    number_of_banks: u8,      // db ? ; number of banks
    memory_model: u8,         // db ; memory model type
    bank_size: u8,            // db ? ; bank size in KB
    number_of_image_pages: u8, // db ; number of images
    reserved1: u8,            // db 1 ; reserved for page function
    /// Direct Color fields (required for direct/6 and YUV/7 memory models)
    red_mask_size: u8, // db ? ; size of direct color red mask in bits
    red_field_position: u8,   // db ? ; bit position of lsb of red mask
    green_mask_size: u8,      // db ? ; size of direct color green mask in bits
    green_field_position: u8, // db ? ; bit position of lsb of green mask
    blue_mask_size: u8,       // db ? ; size of direct color blue mask in bits
    blue_field_position: u8,  // db ? ; bit position of lsb of blue mask
    rsvd_mask_size: u8,       // db ? ; size of direct color reserved mask in bits
    rsvd_field_position: u8,  // db ? ; bit position of lsb of reserved mask
    direct_color_mode_info: u8, // db ? ; direct color mode attributes
    /// Mandatory information for VBE 2.0 and above
    phys_base_ptr: u32, // dd ? ; physical address for flat memory frame buffer
    reserved2: u32,           // dd 0 ; Reserved - always set to 0
    reserved3: u16,           // dw 0 ; Reserved - always set to 0
    /// Mandatory information for VBE 3.0 and above
    lin_bytes_per_scan_line: u16, // dw ? ; bytes per scan line for linear modes
    bnk_number_of_image_pages: u8, // db ? ; number of images for banked modes
    lin_number_of_image_pages: u8, // db ? ; number of images for linear modes
    lin_red_mask_size: u8,    // db ? ; size of direct color red mask (linear modes)
    lin_red_field_position: u8, // db ? ; bit position of lsb of red mask (linear modes)
    lin_green_mask_size: u8,  // db ? ; size of direct color green mask (linear modes)
    lin_green_field_position: u8, //db // ? ? ; bit position of lsb of green mask (linear modes)
    lin_blue_mask_size: u8,   // db ? ; size of direct color blue mask (linear modes)
    lin_blue_field_position: u8, // db ? ; bit position of lsb of blue mask (linear modes)
    lin_rsvd_mask_size: u8,   // db ? ; size of direct color reserved mask (linear modes)
    lin_rsvd_field_position: u8, // db ? ; bit position of lsb of reserved mask (linear modes)
    max_pixel_clock: u32,     // dd ? ; maximum pixel clock (in Hz) for graphics mode
    reserved4: ModeInfoReserved4, //db 189 dup (?) ; remainder of ModeInfo
}}

define_raw_data!(ModeInfoReserved4, 189);

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[repr(packed)]
pub struct CrtcInfo {{
    horizontal_total: u16,      //dw ?  ; Horizontal total in pixels
    horizontal_sync_start: u16, //dw ?  ; Horizontal sync start in pixels
    horizontal_sync_end: u16,   //dw ?  ; Horizontal sync end in pixels
    vertical_total: u16,        //dw ?  ; Vertical total in lines
    vertical_sync_start: u16,   //dw ?  ; Vertical sync start in lines
    vertical_sync_end: u16,     //dw ?  ; Vertical sync end in lines
    flags: u8,                  //db ?  ; Flags (Interlaced, Double Scan etc)
    pixel_clock: u32,           //dd ?  ; Pixel clock in units of Hz
    refresh_rate: u16,          //dw ?  ; Refresh rate in units of 0.01 Hz
    reserved: CrtcInfoReserved, //db 40 dup (?) ; remainder of mode_info_block
}}

define_raw_data!(CrtcInfoReserved, 40);

static mut VBE_INFO: Option<VbeInfo> = None;

static mut MODE_INFO: Option<ModeInfo> = None;

static mut CRTC_INFO: Option<CrtcInfo> = None;

extern \"C\" {{
    static _font: Font;
}}

/// structure contains font for the 255 ascii char
#[repr(C)]
struct Font(pub [u8; 16 * 256]);

impl Font {{
    /// return the 16 * u8 slice font corresponding to the char
    fn get_char(&self, c: u8) -> &[u8] {{
        &self.0[c as usize * 16..(c as usize + 1) * 16]
    }}
}}

#[derive(Debug, Copy, Clone)]
pub struct RGB(pub u32);

impl From<TextColor> for RGB {{
    fn from(c: TextColor) -> Self {{
        match c {{
            TextColor::Red => RGB(0xFF0000),
            TextColor::Green => RGB(0x00FF00),
            TextColor::Blue => RGB(0x0000FF),
            TextColor::Yellow => RGB(0xFFFF00),
            TextColor::Cyan => RGB(0x00FFFF),
            TextColor::Brown => RGB(0xA52A2A),
            TextColor::Magenta => RGB(0xFF00FF),
            TextColor::White => RGB(0xFFFFFF),
        }}
    }}
}}

#[derive(Debug, Copy, Clone)]
pub struct VbeMode {{
    memory_location: usize,
    /// in pixel
    width: usize,
    /// in pixel
    height: usize,
    /// in bytes
    bytes_per_pixel: usize,
    /// in pixel
    x: usize,
    /// in pixel
    y: usize,
    /// in pixel
    char_height: usize,
    /// in pixel
    char_width: usize,
    mode: u16,
    text_color: RGB,
}}

impl VbeMode {{
    pub fn new(memory_location: usize, width: usize, height: usize, bpp: usize) -> Self {{
        Self {{
            memory_location,
            width,
            height,
            bytes_per_pixel: 24 / 8,
            mode: 0,
            x: 0,
            y: 0,
            char_width: 8,
            char_height: 16,
            text_color: TextColor::White.into(),
        }}
    }}
    /// return window size in nb char
    pub fn query_window_size(&self) -> (usize, usize) {{
        (self.height / self.char_height, self.width / self.char_width)
    }}
    fn put_pixel(&self, y: usize, x: usize, color: RGB) {{
        unsafe {{
            *((self.memory_location + y * self.width * self.bytes_per_pixel + x * self.bytes_per_pixel) as *mut u32) =
                color.0;
        }}
    }}
    fn put_pixel_lin(&self, pos: usize, color: RGB) {{
        unsafe {{
            *((self.memory_location + pos * self.bytes_per_pixel) as *mut RGB) = color;
        }}
    }}
    pub fn fill_screen(&self, color: RGB) {{
        for p in 0..self.width * self.height {{
            self.put_pixel_lin(p, color);
        }}
    }}
}}

impl Drawer for VbeMode {{
    fn draw_character(&self, c: char, cursor_y: usize, cursor_x: usize) {{
        let char_font;
        unsafe {{
            char_font = _font.get_char(c as u8);
        }}
        let mut y = cursor_y * self.char_height;
        let mut x;
        for l in char_font {{
            x = cursor_x * self.char_width;
            for shift in (0..8).rev() {{
                if *l & 1 << shift != 0 {{
                    self.put_pixel(y, x, self.text_color);
                }}
                x += 1;
            }}
            y += 1;
        }}
    }}
    fn scroll_screen(&self) {{
        unsafe {{
            let line_size = self.char_height * self.width * self.bytes_per_pixel;
            (self.memory_location as *mut u8).copy_from((self.memory_location + line_size) as *const u8, self.bytes_per_pixel *self.width * self.height - line_size);
            ((self.memory_location + self.bytes_per_pixel *self.width * self.height - line_size) as *mut u8).write_bytes(0, line_size);
        }}
    }}
    fn clear_screen(&mut self) {{
        unsafe {{
            (self.memory_location as *mut u8).write_bytes(0, self.bytes_per_pixel * self.width * self.height);
        }}
    }}
    fn set_text_color(&mut self, color: TextColor) -> IoResult {{
        self.text_color = color.into();
        Ok(())
    }}
}}

fn real_mode_op(reg: BaseRegisters, bios_int: u16) -> core::result::Result<(), VbeError> {{
    /*
     ** AL == 4Fh: ** Function is supported
     ** AH == 00h: Function call successful
     */

    unsafe {{
        let res = _real_mode_op(reg, bios_int);
        if res & 0xFF != 0x4F || res & 0xFF00 != 0x00 {{
            Err(res.into())
        }} else {{
            Ok(())
        }}
    }}
}}

fn save_vbe_info() -> core::result::Result<(), VbeError> {{
    unsafe {{
        // VBE 3.0 specification says to put 'VBE2' in vbe_signature field to have pointers
        // points to reserved field instead of far pointer. So in practice it doesn't work
        TEMPORARY_PTR_LOCATION.copy_from(\"VBE2\".as_ptr(), 4);
    }}
    let reg: BaseRegisters = BaseRegisters {{ edi: TEMPORARY_PTR_LOCATION as u32, eax: 0x4f00, ..Default::default() }};
    real_mode_op(reg, 0x10)?;
    unsafe {{ VBE_INFO = Some(VbeInfo::new(TEMPORARY_PTR_LOCATION as *const VbeInfo)) }}
    Ok(())
}}

fn query_mode_info(mode_number: u16) -> Result<ModeInfo, VbeError> {{
    let reg: BaseRegisters = BaseRegisters {{
        edi: TEMPORARY_PTR_LOCATION as u32,
        eax: 0x4f01,
        ecx: mode_number as u32,
        ..Default::default()
    }};
    unsafe {{ real_mode_op(reg, 0x10).map(|_| *(TEMPORARY_PTR_LOCATION as *const ModeInfo)) }}
}}

fn set_vbe_mode(mode_number: u16) -> Result<(), VbeError> {{
    let reg: BaseRegisters = BaseRegisters {{
        edi: TEMPORARY_PTR_LOCATION as u32,
        eax: 0x4f02,
        ebx: (mode_number | 1 << 14) as u32, // set the bit 14 (from 0) to use linear frame buffer
        ..Default::default()
    }};
    real_mode_op(reg, 0x10)?;
    unsafe {{
        CRTC_INFO = Some(*(TEMPORARY_PTR_LOCATION as *const CrtcInfo));
    }}
    Ok(())
}}

/// do all nessesary initialisation and switch to vbe mode 'mode' if given, if not swith to the best resolution mode
pub fn init_graphic_mode(mode: Option<u16>) -> Result<VbeMode, VbeError> {{
    unsafe {{
        save_vbe_info()?;
        match mode {{
            Some(m) => {{
                MODE_INFO = Some(query_mode_info(m)?);
                set_vbe_mode(m)?;
            }}
            None => {{
                let (m, mode_info) = VBE_INFO.unwrap().find_best_resolution_mode();
                MODE_INFO = Some(mode_info);
                set_vbe_mode(m)?;
            }}
        }}
        let mode_info: &ModeInfo = &MODE_INFO.unwrap();
        Ok(VbeMode::new(
            mode_info.phys_base_ptr as usize,
            mode_info.x_resolution as usize,
            mode_info.y_resolution as usize,
            mode_info.bits_per_pixel as usize,
        ))
    }}
}}

#[derive(Debug, Copy, Clone)]
pub enum VbeError {{
    ///AH == 01h:
    Failed,
    ///AH == 02h:
    NotSupportedCurrentConfig,
    ///AH == 03h:
    InvalidCurentMode,
    ///Unknown Error
    Unknown,
}}

impl From<u16> for VbeError {{
    fn from(err_code: u16) -> Self {{
        match err_code & 0xFF00 {{
            0x0100 => VbeError::Failed,
            0x0200 => VbeError::NotSupportedCurrentConfig,
            0x0300 => VbeError::InvalidCurentMode,
            _ => VbeError::Unknown,
        }}
    }}
}}
");
    }
    println!("from {}", function!());

    println!("irqs state: {}", interrupts::get_interrupts_state());
    let _keyboard_port = Pio::<u8>::new(0x60);
    use crate::io::Pio;

    println!("irq mask: {:b}", PIC_8259.get_masks());

    /*
    unsafe {
        assert_eq!(_idt, interrupts::get_idtr().get_interrupt_table());
        for (index, gate) in interrupts::get_idtr().get_interrupt_table().as_slice()[..48].iter().enumerate() {
            println!("{}: {:?}", index, gate);
        }
    }
    */
    let eflags = crate::registers::Eflags::get_eflags();
    //println!("idtr: {:x?}", interrupts::get_idtr());
    println!("{}", eflags);
    println!("{:x?}", eflags);

    println!("from {}", function!());
    println!("{:?} ms ellapsed", debug::bench_end());
    unsafe {
        PIT0.start_at_frequency(18.).unwrap();
    }
    debug::bench_start();
    println!("pit: {:?}", PIT0);
    unsafe {
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
        print!("H");
        SCREEN_MONAD.set_text_color(Color::Red).unwrap();
        print!("E");
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
        print!("L");
        SCREEN_MONAD.set_text_color(Color::Yellow).unwrap();
        print!("L");
        SCREEN_MONAD.set_text_color(Color::Cyan).unwrap();
        print!("O");
        SCREEN_MONAD.set_text_color(Color::Brown).unwrap();
        print!(" ");
        SCREEN_MONAD.set_text_color(Color::Magenta).unwrap();
        print!("W");
        SCREEN_MONAD.set_text_color(Color::White).unwrap();
        print!("O");
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
        print!("R");
        SCREEN_MONAD.set_text_color(Color::Red).unwrap();
        print!("L");
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
        print!("D");
        SCREEN_MONAD.set_text_color(Color::Yellow).unwrap();
        print!(" ");
        SCREEN_MONAD.set_text_color(Color::Cyan).unwrap();
        println!("!");
        SCREEN_MONAD.set_text_color(Color::White).unwrap();
    }
    unsafe {
        SCREEN_MONAD
            .draw_graphic_buffer(|buffer: *mut u8, width: usize, height: usize, bpp: usize| {
                draw_image(&_wanggle_bmp_start, buffer, width, height, bpp)
            })
            .unwrap();
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
    }
    let mut rtc = Rtc::new();
    let date = rtc.read_date();
    println!("{}", date);
    0
}
