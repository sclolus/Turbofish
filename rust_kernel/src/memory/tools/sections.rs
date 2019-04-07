//! contains the symbols defined in the linker and a macro to get their address
extern "C" {
    pub static __start_text: u8;
    pub static __end_text: u8;

    pub static __start_boot: u8;
    pub static __end_boot: u8;

    pub static __start_rodata: u8;
    pub static __end_rodata: u8;

    pub static __start_data: u8;
    pub static __end_data: u8;

    pub static __start_debug: u8;
    pub static __end_debug: u8;

    pub static __start_bss: u8;
    pub static __end_bss: u8;
    pub static virtual_offset: u8;
    pub static kernel_virtual_end: u8;
    pub static kernel_physical_start: u8;
    pub static kernel_physical_end: u8;
}

/// get the symbol addr
#[macro_use]
macro_rules! symbol_addr {
    ($ident: ident) => {
        #[allow(unused_unsafe)]
        unsafe {
            &$ident as *const _ as usize
        }
    };
}
// #[macro_use]
// macro_rules! print_section {
//     ($ident: ident) => {
//         println!(
//             "{}: [{:p}: {:p}[",
//             stringify!($ident),
//             &concat_idents!(__, start_, $ident),
//             &concat_idents!(__, end_, $ident)
//         );
//     };
// }

// #[macro_use]
// macro_rules! get_section_tuple {
//     ($ident: ident) => {
//         (
//             &concat_idents!(__, start_, $ident) as *const _ as usize,
//             &concat_idents!(__, end_, $ident) as *const _ as usize,
//         )
//     };
// }

// #[macro_use]
// macro_rules! sections {
//     () => {
//         [
//             ("text", get_section_tuple!(text)),
//             ("boot", get_section_tuple!(boot)),
//             ("bss", get_section_tuple!(bss)),
//             ("rodata", get_section_tuple!(rodata)),
//             ("data", get_section_tuple!(data)),
//             ("debug", get_section_tuple!(debug)),
//         ]
//         .iter()
//     };
// }
