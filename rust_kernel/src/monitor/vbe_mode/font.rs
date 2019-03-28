/// structure contains font for the 255 ascii char
#[repr(C)]
// TODO Must be declared dynamiquely and remove 16 magic
pub struct Font(pub [u8; 16 * 256]);

impl Font {
    /// return the 16 * u8 slice font corresponding to the char
    pub fn get_char(&self, c: u8) -> &[u8] {
        &self.0[c as usize * 16..(c as usize + 1) * 16]
    }
}

extern "C" {
    pub static _font: Font;
    pub static _font_width: usize;
    pub static _font_height: usize;
}
