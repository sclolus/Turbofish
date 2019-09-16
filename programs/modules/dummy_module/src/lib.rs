#![cfg_attr(not(test), no_std)]

use kernel_modules::{ModuleName, ModuleResult, SymbolList};

#[no_mangle]
//fn _start(_symtab_list: *const SymbolList, _module_type: ModuleName) -> ModuleResult<()> {
fn _start(symtab_list: SymbolList, _module_type: ModuleName) -> ModuleResult<()> {
    (symtab_list.write)("I've never install GNU/Linux.\n");
    Ok(())
}

#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
