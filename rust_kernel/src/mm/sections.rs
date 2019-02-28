#[macro_use]
macro_rules! print_section {
    ($ident: ident) => {
        println!(
            "{}: [{:p}: {:p}[",
            stringify!($ident),
            &concat_idents!(__, start_, $ident),
            &concat_idents!(__, end_, $ident)
        );
    };
}

#[macro_use]
macro_rules! get_section_tuple {
    ($ident: ident) => {
        (
            &concat_idents!(__, start_, $ident) as *const _ as usize,
            &concat_idents!(__, end_, $ident) as *const _ as usize,
        )
    };
}

#[macro_use]
macro_rules! sections {
    () => {
        [
            ("text", get_section_tuple!(text)),
            ("boot", get_section_tuple!(boot)),
            ("bss", get_section_tuple!(bss)),
            ("rodata", get_section_tuple!(rodata)),
            ("data", get_section_tuple!(data)),
            ("debug", get_section_tuple!(debug)),
        ]
        .iter()
    };
}
