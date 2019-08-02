use alloc::vec::Vec;
use core::mem;
use elf_loader::{ElfHeader, ProgramHeader};

/// This structure is the result of the parsing of a ELF file
#[derive(Debug)]
pub struct Elf {
    pub header: ElfHeader,
    pub program_header_table: Vec<ProgramHeader>,
}

/// Parse a ELF file from a slice
pub fn load_elf(content: &[u8]) -> Elf {
    let header = ElfHeader::from_bytes(&content).unwrap();

    let program_header_table = {
        let mut ph_table = Vec::new();

        // println!("{:#X?}", &header);
        use core::slice;
        let program_header_table: &[[u8; mem::size_of::<ProgramHeader>()]] = unsafe {
            slice::from_raw_parts(
                &content[header.program_header_table_offset as usize] as *const u8 as *const _,
                header.nbr_program_header as usize,
            )
        };
        // println!("\nProgram header table:");
        for (_index, program_header) in program_header_table.iter().enumerate() {
            let pheader = ProgramHeader::from_bytes(program_header as &[u8]).unwrap();
            // println!("{}: {:#X?}", index, pheader);
            ph_table.push(pheader);
        }
        ph_table
    };
    Elf { header, program_header_table }
}
