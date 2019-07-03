use super::VbeMode;

use raw_data::define_raw_data;
use registers::BaseRegisters;

const TEMPORARY_PTR_LOCATION: *mut u8 = 0x2000 as *mut u8;

const LINEAR_FRAMEBUFFER_VIRTUAL_ADDR: *mut u8 = 0xf0000000 as *mut u8;

extern "C" {
    pub fn real_mode_op(reg: *mut BaseRegisters, bios_int: u16) -> u16;
    pub fn kreserve(virt: *mut u8, phys: *mut u8, size: usize) -> *mut u8;
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VbeInfo {
    /*0  */ pub vbe_signature: [u8; 4], //db 'VESA' ; VBE Signature
    /*4  */ pub vbe_version: u16, //dw 0300h ; vbe version
    /*6  */ pub oem_string_offset: u32, //dd ? ; vbe_far_offset to oem string
    /*10 */ pub capabilities: u32, //db 4 dup (?) ; capabilities of graphics controller
    /*14 */ pub video_mode_offset: u32, //dd ? ; vbe_far_offset to video_mode_list
    /*18 */
    pub total_memory: u16, //dw ? ; number of 64kb memory blocks added for vbe 2.0+
    /*20 */ pub oem_software_rev: u16, //dw ? ; vbe implementation software revision
    /*22 */
    pub oem_vendor_name_offset: u32, //dd ? ; vbe_far_offset to vendor name string
    /*26 */
    pub oem_product_name_offset: u32, //dd ? ; vbe_far_offset to product name string
    /*30 */
    pub oem_product_rev_offset: u32, //dd ? ; vbe_far_offset to product revision string
    /*34 */
    pub reserved: VbeInfoReserved, //db 222 dup (?) ; reserved for vbe implementation scratch area
    /*256*/ pub oem_data: VbeInfoOemData, //db 256 dup ; data area for oem strings
}

define_raw_data!(VbeInfoReserved, 222);
define_raw_data!(VbeInfoOemData, 256);

#[allow(dead_code)]
impl VbeInfo {
    /// only way to initialize VbeInfo safely transform all the pointers within the struct by their offsets
    unsafe fn new(ptr: *const Self) -> Self {
        Self {
            video_mode_offset: (*ptr).video_mode_offset - ptr as u32,
            ..*ptr
        }
    }
    /// calculate the mode ptr using the address of self and the offset
    fn get_video_mode_ptr(&self) -> *const u16 {
        unsafe {
            (self as *const Self as *const u8).add(self.video_mode_offset as usize) as *const u16
        }
    }
    /// return the number of modes available
    /// The VideoModePtr is a VbeFarPtr that points to a list of mode numbers for all display modes
    /// supported by the VBE implementation. Each mode number occupies one word (16 bits). The list
    /// of mode numbers is terminated by a -1 (0FFFFh). The mode numbers in this list represent all of
    /// the potentially supported modes by the display controller.
    fn nb_mode(&self) -> usize {
        let mut i = 0;
        let video_mode_ptr = self.get_video_mode_ptr();
        unsafe {
            while *((video_mode_ptr).offset(i as isize)) != 0xFFFF {
                i += 1;
                // 111 is the maximum number of modes because reserved is 222 bytes
                if i >= 111 {
                    return i;
                }
            }
        }
        i
    }
    /// return an iterator on available modes
    pub fn iter_modes(&self) -> core::slice::Iter<u16> {
        unsafe { core::slice::from_raw_parts(self.get_video_mode_ptr(), self.nb_mode()).iter() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ModeInfo {
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
}

define_raw_data!(ModeInfoReserved4, 189);

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct CrtcInfo {
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
}

define_raw_data!(CrtcInfoReserved, 40);

fn vbe_real_mode_op(mut reg: BaseRegisters, bios_int: u16) -> core::result::Result<(), VbeError> {
    /*
     ** AL == 4Fh: ** Function is supported
     ** AH == 00h: Function call successful
     */
    unsafe {
        let res = real_mode_op(&mut reg as *mut BaseRegisters, bios_int);
        if res & 0xFF != 0x4F || res & 0xFF00 != 0x00 {
            Err(res.into())
        } else {
            Ok(())
        }
    }
}

unsafe fn save_vbe_info() -> Result<VbeInfo, VbeError> {
    // VBE 3.0 specification says to put 'VBE2' in vbe_signature field to have pointers
    // points to reserved field instead of far pointer. So in practice it doesn't work
    TEMPORARY_PTR_LOCATION.copy_from("VBE2".as_ptr(), 4);
    let reg: BaseRegisters = BaseRegisters {
        edi: TEMPORARY_PTR_LOCATION as u32,
        eax: 0x4f00,
        ..Default::default()
    };
    vbe_real_mode_op(reg, 0x10)?;
    Ok(VbeInfo::new(TEMPORARY_PTR_LOCATION as *const VbeInfo))
}

fn query_mode_info(mode_number: u16) -> Result<ModeInfo, VbeError> {
    let reg: BaseRegisters = BaseRegisters {
        edi: TEMPORARY_PTR_LOCATION as u32,
        eax: 0x4f01,
        ecx: mode_number as u32,
        ..Default::default()
    };
    unsafe { vbe_real_mode_op(reg, 0x10).map(|_| *(TEMPORARY_PTR_LOCATION as *const ModeInfo)) }
}

unsafe fn set_vbe_mode(mode_number: u16) -> Result<CrtcInfo, VbeError> {
    let reg: BaseRegisters = BaseRegisters {
        edi: TEMPORARY_PTR_LOCATION as u32,
        eax: 0x4f02,
        ebx: (mode_number | 1 << 14) as u32, // set the bit 14 (from 0) to use linear frame buffer
        ..Default::default()
    };
    vbe_real_mode_op(reg, 0x10)?;
    Ok(*(TEMPORARY_PTR_LOCATION as *const CrtcInfo))
}

/// do all nessesary initialisation and switch to vbe mode 'mode' if given, if not swith to the best resolution mode
pub fn init_graphic_mode(mode: u16) -> Result<VbeMode, VbeError> {
    unsafe {
        let _vbe_info = save_vbe_info()?;
        let mode_info: ModeInfo = query_mode_info(mode)?;

        if kreserve(
            LINEAR_FRAMEBUFFER_VIRTUAL_ADDR,
            mode_info.phys_base_ptr as *mut u8,
            mode_info.x_resolution as usize
                * mode_info.y_resolution as usize
                * mode_info.bits_per_pixel as usize
                / 8,
        ) == 0 as *mut u8
        {
            panic!("reserve failed")
        }

        /*
         * Make all Dynamics allocations before switching to new graphic mode
         */
        let mut ret = VbeMode::new(
            LINEAR_FRAMEBUFFER_VIRTUAL_ADDR,
            mode_info.x_resolution as usize,
            mode_info.y_resolution as usize,
            mode_info.bits_per_pixel as usize,
            mode_info,
        );
        ret.crtc_info = Some(set_vbe_mode(mode)?);
        Ok(ret)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum VbeError {
    ///AH == 01h:
    Failed,
    ///AH == 02h:
    NotSupportedCurrentConfig,
    ///AH == 03h:
    InvalidCurentMode,
    ///Unknown Error
    Unknown,
}

impl From<u16> for VbeError {
    fn from(err_code: u16) -> Self {
        match err_code & 0xFF00 {
            0x0100 => VbeError::Failed,
            0x0200 => VbeError::NotSupportedCurrentConfig,
            0x0300 => VbeError::InvalidCurentMode,
            _ => VbeError::Unknown,
        }
    }
}
