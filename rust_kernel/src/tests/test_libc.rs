use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;

#[no_mangle]
pub extern "C" fn kmain(
    multiboot_info: *const MultibootInfo,
    device_map_ptr: *const DeviceMap,
) -> ! {
    crate::rust_main::init_kernel(multiboot_info, device_map_ptr);
    crate::taskmaster::start(
        "/bin/init",
        &[
            "/bin/init",
            "/bin/DeepTests/MasterDeepThought",
            "/bin/DeepTests/DeepThought",
        ],
        &[],
    );
}
