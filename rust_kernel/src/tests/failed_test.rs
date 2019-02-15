use crate::multiboot::MultibootInfo;

#[no_mangle]
pub extern "C" fn kmain(_multiboot_info: *const MultibootInfo) -> u32 {
    let _a: Result<(), &str> = Err("error").unwrap();
    0
}
