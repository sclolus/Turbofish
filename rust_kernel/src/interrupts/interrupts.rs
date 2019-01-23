use crate::ffi::*;

extern "C" {
    pub fn _isr_keyboard() -> ();
    pub fn _cli();
    pub fn _sli();
}


#[no_mangle]
extern "C" fn generic_interrupt_handler(interrupt_name: *const u8) {
    println!("in interrupt context");
    unsafe  {
    let slice: &[u8] = core::slice::from_raw_parts(interrupt_name, strlen(interrupt_name as *const c_char));
        println!("From interrupt: {}", core::str::from_utf8_unchecked(slice)) // Make str slice (&[str]) with &[u8]
    }
}
