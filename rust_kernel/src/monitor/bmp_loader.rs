use crate::ffi::c_char;
use core::slice;

#[derive(Debug, Copy, Clone)]
pub enum BmpError {
    BmpNotSupported,
    BppNotSupported,
}

pub type BmpResult = core::result::Result<*mut u8, BmpError>;

const GRAPHIC_BUFFER_LOCATION: *mut u8 = 0x2000000 as *mut u8;

extern "C" {
    static _binary_medias_asterix_bmp_start: BmpImage;
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(packed)]
pub struct BmpImage {
    /*0  */ signature: [c_char; 2],
    /*2  */ filesize: u32,
    /*6  */ reserved: u32,
    /*10 */ fileoffset_to_pixelarray: u32,

    /*14 */ dibheadersize: u32,
    /*18 */ width: u32,
    /*22 */ height: u32,
    /*24 */ planes: u16,
    /*26 */ bitsperpixel: u16,
    /*28 */ compression: u32,
    /*32 */ imagesize: u32,
    /*36 */ ypixelpermeter: u32,
    /*40 */ xpixelpermeter: u32,
    /*44 */ numcolorspallette: u32,
    /*48 */ mostimpcolor: u32,
}

// Last pixel line of bitmap format is the first line of standard screen format
fn fill_image(output: *mut u8, image: *const u8, width: usize, height: usize, bpp: usize, header: BmpImage) {
    let ptr_input = unsafe { slice::from_raw_parts(image, header.filesize as usize) };
    let ptr_output = unsafe { slice::from_raw_parts_mut(output, width * height * bpp / 8 as usize) };

    // offset to last input line
    let mut input_index = (header.height - 1) as usize * header.width as usize * 3;

    for (i, elem) in ptr_output.iter_mut().enumerate() {
        if bpp == 32 && (i % 4) == 3 {
            continue;
        }
        *elem = ptr_input[input_index];
        input_index += 1;
        // check if on end of pixel line
        if (input_index % (header.width as usize * 3)) == 0 && input_index != header.width as usize * 3 {
            input_index -= header.width as usize * 3 * 2;
        }
    }
}

// This function implemente no scale change, only work with 1024 * 768 * (24b || 32b bitmap)
pub fn load_image_buffer(width: usize, height: usize, bpp: usize) -> BmpResult {
    if bpp != 32 && bpp != 24 {
        Err(BmpError::BppNotSupported)
    } else {
        let header = unsafe { _binary_medias_asterix_bmp_start };
        if header.bitsperpixel != 24 && header.width != 1024 && header.height != 768 {
            Err(BmpError::BmpNotSupported)
        } else {
            let ptr = unsafe { &_binary_medias_asterix_bmp_start as *const BmpImage as *const u8 };

            fill_image(
                GRAPHIC_BUFFER_LOCATION,
                unsafe { ptr.add(header.fileoffset_to_pixelarray as usize) },
                width,
                height,
                bpp,
                header,
            );
            Ok(GRAPHIC_BUFFER_LOCATION)
        }
    }
}
