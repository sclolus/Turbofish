use crate::multiboot::MultibootInfo;

#[no_mangle]
pub extern "C" fn kmain(_multiboot_info: *const MultibootInfo) -> ! {
    assert_eq!(1, 0);
    loop {}
}
