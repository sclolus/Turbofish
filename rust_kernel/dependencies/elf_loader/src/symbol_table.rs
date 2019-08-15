//! This file contrains extra tool to debug user space programms loaded by Elf

/// Extract for https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-79797.html
///
/// A briefing about the .symtab section:
/// -------------------------------------
/// A symbol table entry has the following format. See sys/elf.h.
/// typedef struct {
///     Elf32_Word      st_name;    // (4)
///     Elf32_Addr      st_value;   // (4)
///     Elf32_Word      st_size;    // (4)
///     unsigned char   st_info;    // (1)
///     unsigned char   st_other;   // (1)
///     Elf32_Half      st_shndx;   // (2)
/// } Elf32_Sym;                    // (16)
///
/// typedef uint32_t Elf32_Addr;
/// typedef uint16_t Elf32_Half;
/// typedef uint32_t Elf32_Off;
/// typedef int32_t Elf32_Sword;
/// typedef uint32_t Elf32_Word;
/// typedef uint64_t Elf32_Lword;
///
/// st_name:
/// --------
/// An index into the object file's symbol string table, which holds the character representations
/// of the symbol names. If the value is nonzero, the value represents a string table index that
/// gives the symbol name. Otherwise, the symbol table entry has no name.
/// To find name, apply the name offset on the .strtab section of the ELF file
///
/// st_value:
/// ---------
/// The value of the associated symbol. The value can be an absolute value or an address, depending on the context. See Symbol Values.
///
/// st_size:
/// --------
/// Many symbols have associated sizes. For example, a data object's size is the number of bytes that are contained in the object.
/// This member holds the value zero if the symbol has no size or an unknown size.
///
/// st_info:
/// --------
/// The symbol's type and binding attributes. A list of the values and meanings appears in Table 12-18. The following code shows
/// how to manipulate the values. See sys/elf.h.
///
/// #define ELF32_ST_BIND(info)          ((info) >> 4)
/// #define ELF32_ST_TYPE(info)          ((info) & 0xf)
/// #define ELF32_ST_INFO(bind, type)    (((bind)<<4)+((type)&0xf))
///
/// STB_LOCAL     0
/// STB_GLOBAL    1
/// STB_WEAK      2
/// STB_LOOS     10
/// STB_HIOS     12
/// STB_LOPROC   13
/// STB_HIPROC   15
///
/// st_other:
/// ---------
/// A symbol's visibility. A list of the values and meanings appears in Table 12-20. The following code shows how to manipulate the
/// values for both 32–bit objects and 64–bit objects. Other bits are set to zero, and have no defined meaning.
/// #define ELF32_ST_VISIBILITY(o)       ((o)&0x3)
///
/// st_shndx:
/// ---------
/// Every symbol table entry is defined in relation to some section. This member holds the relevant section header table index.
/// Some section indexes indicate special meanings. See Table 12-4.
use super::{ElfHeader, SectionHeader, SectionHeaderType};

use alloc::string::String;
use alloc::vec::Vec;

use core::{mem, slice};

/// This structure represents one symbol entry in a .symtab section in elf file
#[derive(Debug)]
struct ElfSymbolEntry {
    name: u32,
    value: u32,
    size: u32,
    info: u8,
    other: u8,
    shndx: u16,
}

/// This structure represents one symbol entry in human readable format style
#[derive(Debug)]
pub struct SymbolEntry {
    name: String,
    addr: u32,
    size: usize,
    info: u8,
}

/// Main structure and his famous Debug boilerplate
#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<SymbolEntry>,
}

/// Main implementation
impl SymbolTable {
    /// This function need the ELF content
    // TODO: Error management is anonymous, change that
    pub fn try_new(content: &[u8]) -> Result<Self, ()> {
        // First get the header of the ELF file
        let header = ElfHeader::from_bytes(&content).map_err(|_| ())?;

        // Get the index of the .shstrtab
        let shstrtab_index = header.section_header_str_index;

        // Then get the section header table slice
        let section_header_table: &[[u8; mem::size_of::<SectionHeader>()]] = unsafe {
            core::slice::from_raw_parts(
                &content[header.section_header_table_offset as usize] as *const u8 as *const _,
                header.nbr_section_header as usize,
            )
        };

        // Get the three above sections
        let mut symtab: Option<SectionHeader> = None;
        let mut strtab: Option<SectionHeader> = None;
        let mut shstrtab: Option<SectionHeader> = None;
        for (index, section_header) in section_header_table.iter().enumerate() {
            let sheader = SectionHeader::from_bytes(section_header as &[u8]).map_err(|_| ())?;
            if sheader.sh_type == SectionHeaderType::Symtab {
                symtab = Some(sheader);
            } else if sheader.sh_type == SectionHeaderType::Strtab {
                // Make difference between shstrtab and strtab
                if index == shstrtab_index as _ {
                    shstrtab = Some(sheader);
                } else {
                    strtab = Some(sheader);
                }
            }
        }

        // Continue only if we have found all the necessary header sections
        if let (Some(symtab), Some(strtab), Some(_shstrtab)) = (symtab, strtab, shstrtab) {
            let mut symbols: Vec<SymbolEntry> = Vec::new();

            let symtab_offset = symtab.sh_offset as usize;
            let symtab_size = symtab.sh_size as usize;

            let s = unsafe {
                slice::from_raw_parts(
                    &content[symtab_offset] as *const _ as *const ElfSymbolEntry,
                    symtab_size / mem::size_of::<ElfSymbolEntry>(),
                )
            };
            // Iterate over the raw ElfSymbolEntry slice and push founded elems into the symbols Vector
            for elem in s.iter() {
                // Check if the symbal has no name
                if elem.name == 0 {
                    // Just skip it
                    continue;
                }
                let raw_symbol_name = unsafe {
                    // Take a len of the string size (in C style)
                    let mut size = 0;
                    while content[strtab.sh_offset as usize + elem.name as usize + size] != 0 {
                        size += 1;
                    }
                    core::str::from_utf8(slice::from_raw_parts(
                        &content[strtab.sh_offset as usize + elem.name as usize],
                        size,
                    ))
                    .map_err(|_| ())?
                };
                let mut symbol_name: String = String::new();
                symbol_name.try_reserve_exact(raw_symbol_name.len()).map_err(|_| ())?;
                symbol_name.insert_str(0, raw_symbol_name);

                // Finally push a Rust SymbolEntry
                symbols.try_reserve(1).map_err(|_| ())?;
                symbols.push(SymbolEntry {
                    name: symbol_name,
                    addr: elem.value,
                    size: elem.size as _,
                    // TODO: The current implementation does not care about symbol info
                    info: elem.info,
                });
            }

            Ok(Self { symbols })
        } else {
            Err(())
        }
    }

    /// Get the symbol name corresponding to an EIP value
    pub fn get_symbol_name(&self, eip: u32) -> Option<&String> {
        for elem in self.symbols.iter() {
            if eip >= elem.addr && eip < elem.addr + elem.size as u32 {
                return Some(&elem.name);
            }
        }
        None
    }
}
