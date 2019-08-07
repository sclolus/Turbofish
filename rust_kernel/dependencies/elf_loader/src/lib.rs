#![cfg_attr(all(not(test), not(feature = "std-print")), no_std)]
use bitflags::bitflags;
use core::convert::{AsRef, TryFrom, TryInto};
use core::mem;
use core::result::Result as CoreResult;

#[macro_use]
extern crate derive_is_enum_variant;

#[cfg(not(feature = "std-print"))]
#[allow(unused_imports)]
#[macro_use]
#[cfg(not(test))]
extern crate terminal;

extern crate alloc;

use alloc::vec::Vec;

pub struct ElfParser<'a> {
    file: &'a [u8],
    elf_header: Option<ElfHeader>,
    program_headers: Vec<ProgramHeader>,
    section_headers: Vec<SectionHeader>,
    string_table: Option<&'a [u8]>,
}

impl<'a> TryFrom<&'a [u8]> for ElfParser<'a> {
    type Error = ElfParseError;

    fn try_from(value: &'a [u8]) -> Result<Self> {
        Ok(Self::new(value))
    }
}

type Result<T> = CoreResult<T, ElfParseError>;

impl<'a> ElfParser<'a> {
    pub fn new(file: &'a [u8]) -> Self {
        Self { file, elf_header: None, program_headers: Vec::new(), section_headers: Vec::new(), string_table: None }
    }

    pub fn parse(&mut self) -> Result<()> {
        let header = ElfHeader::from_bytes(self.file)?;

        if header.program_header_table_size != 0 {
            let ph_size = header.program_header_table_size as usize;
            let ph_table_size = ph_size * header.nbr_program_header as usize;
            let ph_table_start = header.program_header_table_offset as usize;
            let ph_table_end = ph_table_start + ph_table_size;

            let ph_table = &self.file[ph_table_start..ph_table_end];

            for ph in ph_table.chunks(ph_size) {
                self.program_headers.push(ph.try_into()?)
            }
        }

        if header.section_header_table_size != 0 {
            let sh_size = header.section_header_table_size as usize;
            let sh_table_size = sh_size * header.nbr_section_header as usize;
            let sh_table_start = header.section_header_table_offset as usize;
            let sh_table_end = sh_table_start + sh_table_size;

            let sh_table = &self.file[sh_table_start..sh_table_end];

            for sh in sh_table.chunks(sh_size) {
                self.program_headers.push(sh.try_into()?)
            }
        }

        self.elf_header = Some(header);
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, is_enum_variant)]
pub enum ElfParseError {
    BadMagic,
    InvalidHeader,
    InvalidEndian,
    InvalidFormat,
    InvalidAbi,
    InvalidObjectType,
    InvalidTargetArchitecture,
    InvalidSegmentType,
    InvalidProgramHeader,
    InvalidProgramHeaderNumber,
    InvalidProgramHeaderFlags,
    InvalidSegmentAlignment,
    InvalidSectionHeader,
    InvalidSectionHeaderType,
    InvalidSectionAlignment,
    InvalidSectionHeaderNumber,
    InvalidSectionHeaderFlags,
    InvalidSymbol,
    InvalidSymbolType,
    InvalidSymbolBinding,
    InvalidSymbolVisibility,
    InvalidRel,
    InvalidRela,
}

use core::array::TryFromSliceError;
impl From<TryFromSliceError> for ElfParseError {
    fn from(_value: TryFromSliceError) -> Self {
        ElfParseError::InvalidHeader
    }
}

fn copy_to_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut array = Default::default();

    <A as AsMut<[T]>>::as_mut(&mut array).copy_from_slice(slice);
    array
}

fn slice_to_u32(slice: &[u8]) -> u32 {
    u32::from_ne_bytes(copy_to_array(&slice[0x0..0x4]))
}

fn slice_to_u16(slice: &[u8]) -> u16 {
    u16::from_ne_bytes(copy_to_array(&slice[0x0..0x2]))
}

#[derive(Debug, Copy, Clone)]
pub enum Endian {
    Little,
    Big,
}

impl TryFrom<u8> for Endian {
    type Error = ElfParseError;

    fn try_from(value: u8) -> Result<Self> {
        use Endian::*;
        match value {
            1 => Ok(Little),
            2 => Ok(Big),
            _ => Err(ElfParseError::InvalidEndian),
        }
    }
}

#[derive(Debug, Copy, Clone, is_enum_variant)]
pub enum Format {
    Bit32,
    Bit64,
}

impl TryFrom<u8> for Format {
    type Error = ElfParseError;

    fn try_from(value: u8) -> Result<Self> {
        use Format::*;
        match value {
            1 => Ok(Bit32),
            2 => Ok(Bit64),
            _ => Err(ElfParseError::InvalidFormat),
        }
    }
}

#[derive(Debug, Copy, Clone, is_enum_variant)]
pub enum Abi {
    SystemV,
    HPUX,
    NetBSD,
    Linux,
    Hurd,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    Tru64,
    NovellModesto,
    OpenBSD,
    OpenVMS,
    NonStopKernel,
    AROS,
    FenixOs,
    CloudABI,
}

impl TryFrom<u8> for Abi {
    type Error = ElfParseError;

    fn try_from(value: u8) -> Result<Self> {
        use Abi::*;
        Ok(match value {
            0x0 => SystemV,
            0x1 => HPUX,
            0x2 => NetBSD,
            0x3 => Linux,
            0x4 => Hurd,
            0x6 => Solaris,
            0x7 => AIX,
            0x8 => IRIX,
            0x9 => FreeBSD,
            0xA => Tru64,
            0xB => NovellModesto,
            0xC => OpenBSD,
            0xD => OpenVMS,
            0xE => NonStopKernel,
            0xF => AROS,
            0x11 => FenixOs,
            0x10 => CloudABI,
            _ => return Err(ElfParseError::InvalidAbi),
        })
    }
}

#[derive(Debug, Copy, Clone, is_enum_variant)]
pub enum ObjectType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Loos,
    Hios,
    Loproc,
    Hiproc,
}

impl TryFrom<u16> for ObjectType {
    type Error = ElfParseError;

    fn try_from(value: u16) -> Result<Self> {
        use crate::ObjectType::*;
        Ok(match value {
            0x0 => None,
            0x1 => Rel,
            0x2 => Exec,
            0x3 => Dyn,
            0x4 => Core,
            0xfe00 => Loos,
            0xfeff => Hios,
            0xff00 => Loproc,
            0xffff => Hiproc,
            _ => return Err(ElfParseError::InvalidObjectType),
        })
    }
}

#[derive(Debug, Copy, Clone, is_enum_variant)]
pub enum Architecture {
    None,
    SPARC,
    X86,
    MIPS,
    PowerPC,
    S390,
    ARM,
    SuperH,
    IA64,
    X86_64,
    AArch64,
    RISCV,
}

impl TryFrom<u16> for Architecture {
    type Error = ElfParseError;

    fn try_from(value: u16) -> Result<Self> {
        use crate::Architecture::*;
        Ok(match value {
            0x0 => None,
            0x2 => SPARC,
            0x3 => X86,
            0x8 => MIPS,
            0x14 => PowerPC,
            0x16 => S390,
            0x28 => ARM,
            0x2A => SuperH,
            0x32 => IA64,
            0x3E => X86_64,
            0xB7 => AArch64,
            0xF3 => RISCV,
            _ => return Err(ElfParseError::InvalidTargetArchitecture),
        })
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct ElfHeader {
    /// 32-bit 64-bit.
    format: Format,

    /// Endianness of this object file.
    endian: Endian,

    /// Identifies the target operating system ABI.
    target_abi: Abi,

    /// Further specifies the ABI.
    abi_version: u8,

    /// The object type of file.
    object_type: ObjectType,

    /// The target architecture of this object file.
    machine: Architecture,

    /// Version of ELF used. Probably 1.
    version: u32,

    /// The address of the entry point. i.e the address in memory where the programs starts executing.
    /// If the file has no associated entry point, this member holds zero.
    pub entry_point: u32,

    /// Offset of the start of the program header table.
    /// If the file has no program header table, this member holds zero.
    pub program_header_table_offset: u32,

    /// Offset of the start of the section header table.
    /// If the file has no section header table, this member holds zero.
    pub section_header_table_offset: u32,

    /// The interpretation of this field depends on the target architecture.
    flags: u32,

    /// The size of this header in bytes.
    self_size: u16,

    /// Contains the size of a program header table entry.
    /// All entries are the same size.
    program_header_table_size: u16,

    /// Contains the number of entries in the program header table.
    pub nbr_program_header: u16,

    /// Contains the size of a section header table entry.
    /// All entries are the same size.
    pub section_header_table_size: u16,

    /// Contains the number of entries in the section header table.
    pub nbr_section_header: u16,

    /// Contains index of the section header table entry that contains the section names.
    section_header_str_index: u16,
}

pub trait ElfDataStructure: Sized {}
pub trait FromBytes<'a>: ElfDataStructure {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self>;
}

impl<'a> ElfDataStructure for ElfParser<'a> {}
impl ElfDataStructure for ElfHeader {}
impl ElfDataStructure for ProgramHeader {}
impl ElfDataStructure for SectionHeader {}
impl ElfDataStructure for Symbol {}
impl ElfDataStructure for Rel {}
impl ElfDataStructure for Rela {}

impl<'a, T> FromBytes<'a> for T
where
    T: TryFrom<&'a [u8], Error = ElfParseError> + ElfDataStructure,
{
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        Self::try_from(bytes)
    }
}

impl TryFrom<&[u8]> for ElfHeader {
    type Error = ElfParseError;

    fn try_from(value: &[u8]) -> Result<Self> {
        // Special case of nbr_program_header, currently not supported.
        const PN_XNUM: u16 = 0xffff;
        const SHN_LORESERVE: u16 = 0xff00;

        if value.len() < mem::size_of::<Self>() {
            return Err(ElfParseError::InvalidHeader);
        }
        // check the magic
        if value[0..4] != [0x7F, 0x45, 0x4c, 0x46] {
            return Err(ElfParseError::BadMagic);
        }

        let nbr_program_header = u16::from_ne_bytes(TryFrom::try_from(&value[0x2C..0x2E])?);

        if nbr_program_header == PN_XNUM {
            return Err(ElfParseError::InvalidProgramHeaderNumber);
        }

        let nbr_section_header = u16::from_ne_bytes(TryFrom::try_from(&value[0x30..0x32])?);

        if nbr_section_header >= SHN_LORESERVE {
            return Err(ElfParseError::InvalidSectionHeaderNumber);
        }

        Ok(Self {
            format: Format::try_from(value[0x4])?,
            endian: Endian::try_from(value[0x5])?,
            target_abi: Abi::try_from(value[0x7])?,
            abi_version: value[0x8],
            object_type: ObjectType::try_from(u16::from_ne_bytes(TryFrom::try_from(&value[0x10..0x12])?))?,
            machine: Architecture::try_from(u16::from_ne_bytes(TryFrom::try_from(&value[0x12..0x14])?))?,
            version: u32::from_ne_bytes(TryFrom::try_from(&value[0x14..0x18])?),
            entry_point: u32::from_ne_bytes(TryFrom::try_from(&value[0x18..0x1C])?),
            program_header_table_offset: u32::from_ne_bytes(TryFrom::try_from(&value[0x1C..0x20])?),
            section_header_table_offset: u32::from_ne_bytes(TryFrom::try_from(&value[0x20..0x24])?),
            flags: u32::from_ne_bytes(TryFrom::try_from(&value[0x24..0x28])?),
            self_size: u16::from_ne_bytes(TryFrom::try_from(&value[0x28..0x2A])?),
            program_header_table_size: u16::from_ne_bytes(TryFrom::try_from(&value[0x2A..0x2C])?),
            nbr_program_header,
            section_header_table_size: u16::from_ne_bytes(TryFrom::try_from(&value[0x2E..0x30])?),
            nbr_section_header,
            section_header_str_index: u16::from_ne_bytes(TryFrom::try_from(&value[0x32..0x34])?),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, is_enum_variant)]
pub enum SegmentType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Loos,
    Hios,
    LoProc,
    HiProc,
    GnuEhFrame,
    GnuStack,
    GnuRelro,
    Losunw,
    Sunwstack,
    ProcessorSpecific,
}

bitflags! {
    #[derive(Default)]
    pub struct ProgramHeaderFlags: u32 {
        #[allow(non_upper_case_globals)]
        const Executable = 0x1;

        #[allow(non_upper_case_globals)]
        const Writable = 0x2;

        #[allow(non_upper_case_globals)]
        const Readable = 0x4;

        #[allow(non_upper_case_globals)]
        const MaskOs = 0x0ff00000;

        #[allow(non_upper_case_globals)]
        const MaskProc = 0xf0000000;
    }
}

impl TryFrom<u32> for ProgramHeaderFlags {
    type Error = ElfParseError;

    fn try_from(value: u32) -> Result<Self> {
        Ok(Self::from_bits(value).ok_or(ElfParseError::InvalidProgramHeaderFlags)?)
    }
}

impl TryFrom<u32> for SegmentType {
    type Error = ElfParseError;

    fn try_from(value: u32) -> Result<Self> {
        use SegmentType::*;
        Ok(match value {
            0x0 => Null,
            0x1 => Load,
            0x2 => Dynamic,
            0x3 => Interp,
            0x4 => Note,
            0x5 => Shlib,
            0x6 => Phdr,
            0x60000000 => Loos,
            0x6FFFFFFF => Hios,
            0x70000000 => LoProc,
            0x7FFFFFFF => HiProc,
            0x6474e550 => GnuEhFrame,
            0x6474e551 => GnuStack,
            0x6474e552 => GnuRelro,
            0x6ffffffa => Losunw,
            0x6ffffffb => Sunwstack,
            _other => ProcessorSpecific,
            // e => {
            //     return Err({
            //         eprintln!("{}", e);
            //         ElfParseError::InvalidSegmentType
            //     });
            // }
        })
    }
}

#[derive(Debug)]
pub struct ProgramHeader {
    pub segment_type: SegmentType,
    /// This member gives the offset from the beginning of the file at which the first byte of thesegment resides
    pub offset: u32,
    /// This member gives the virtual address at which the first byte of the segment resides inmemory.
    pub vaddr: u32,
    /// On systems for which physical addressing is relevant, this member is reserved for thesegment’s physical address.
    pub paddr: u32,
    /// This member gives the number of bytes in the file image of the segment; it may be zero
    pub filez: u32,
    /// This member gives the number of bytes in the memory image of the segment; it may bezero
    pub memsz: u32,
    pub flags: ProgramHeaderFlags,
    /// loadable process segments must have congruent values for vaddr and offset, modulo the page size
    pub align: u32,
}

impl ProgramHeader {
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self> {
        Self::try_from(bytes.as_ref())
    }
}

impl TryFrom<&[u8]> for ProgramHeader {
    type Error = ElfParseError;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < mem::size_of::<Self>() {
            return Err(ElfParseError::InvalidProgramHeader);
        }

        let new = Self {
            segment_type: SegmentType::try_from(slice_to_u32(&value[0x0..0x4]))?,
            offset: slice_to_u32(&value[0x4..0x8]),
            vaddr: slice_to_u32(&value[0x8..0x0C]),
            paddr: slice_to_u32(&value[0x0C..0x10]),
            filez: slice_to_u32(&value[0x10..0x14]),
            memsz: slice_to_u32(&value[0x14..0x18]),
            flags: ProgramHeaderFlags::try_from(slice_to_u32(&value[0x18..0x1C]))?,
            align: slice_to_u32(&value[0x1C..0x20]),
        };

        // if new.p_align > 1 && (new.p_vaddr != new.p_offset % new.p_align || !new.p_align.is_power_of_two()) {
        //     //TODO Loadable  process  segments must have congruent values for
        //     // p_vaddr and p_offset, modulo the page size.
        //     return Err(ElfParseError::InvalidSegmentAlignment);
        // }
        Ok(new)
    }
}

impl core::fmt::Display for ProgramHeader {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(write!(fmt, "type: {:?}, offset: {:08X}, vaddr: {:08X}, paddr: {:08X}, filez: {:08x}, memsz: {:08X}, flags: {:?}, align: {:02X}",
                  self.segment_type,
                  self.offset,
                  self.vaddr,
                  self.paddr,
                  self.filez,
                  self.memsz,
                  self.flags,
                  self.align,
        )?)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, is_enum_variant)]
pub enum SectionHeaderType {
    Null,
    ProgBits,
    Symtab,
    Strtab,
    Rela,
    Hash,
    Dynamic,
    Note,
    NoBits,
    Rel,
    Shlib,
    DynSym,
    InitArray,
    FiniArray,
    PreinitArray,
    Group,
    SymtabShndx,
    Num,
    Loos,
    GnuAttributes,
    GnuHash,
    GnuLibList,
    Checksum,
    Losunw,
    SunwMove,
    SunwComdat,
    SunwSyminfo,
    GnuVerdef,
    GnuVerneed,
    GnuVersym,
    Hisunw,
    Hios,
    LoProc,
    HiProc,
    LoUser,
    HiUser,
}

impl TryFrom<u32> for SectionHeaderType {
    type Error = ElfParseError;

    fn try_from(value: u32) -> Result<Self> {
        use SectionHeaderType::*;
        Ok(match value {
            0x0 => Null,
            0x1 => ProgBits,
            0x2 => Symtab,
            0x3 => Strtab,
            0x4 => Rela,
            0x5 => Hash,
            0x6 => Dynamic,
            0x7 => Note,
            0x8 => NoBits,
            0x9 => Rel,
            0xA => Shlib,
            0xB => DynSym,
            0xE => InitArray,
            0xF => FiniArray,
            0x10 => PreinitArray,
            0x11 => Group,
            0x12 => SymtabShndx,
            0x13 => Num,
            0x60000000 => Loos,
            0x6ffffff5 => GnuAttributes,
            0x6ffffff6 => GnuHash,
            0x6ffffff7 => GnuLibList,
            0x6ffffff8 => Checksum,
            0x6ffffffa => Losunw,
            // 0x6ffffffa => SunwMove,
            0x6ffffffb => SunwComdat,
            0x6ffffffc => SunwSyminfo,
            0x6ffffffd => GnuVerdef,
            0x6ffffffe => GnuVerneed,
            0x6fffffff => GnuVersym,
            // 0x6fffffff => Hisunw,
            // 0x6fffffff => Hios,
            0x70000000 => LoProc,
            0x7fffffff => HiProc,
            0x80000000 => LoUser,
            0x8fffffff => HiUser,
            _ => return Err(ElfParseError::InvalidSectionHeaderType),
        })
    }
}

bitflags! {
    pub struct SectionHeaderFlags: u32 {
        #[allow(non_upper_case_globals)]
        const Write = 0x1;
        #[allow(non_upper_case_globals)]
        const Alloc = 0x2;
        #[allow(non_upper_case_globals)]
        const ExecInstr = 0x4;
        #[allow(non_upper_case_globals)]
        const Merge = 0x10;
        #[allow(non_upper_case_globals)]
        const Strings = 0x20;
        #[allow(non_upper_case_globals)]
        const InfoLink = 0x40;
        #[allow(non_upper_case_globals)]
        const LinkOrder = 0x80;
        #[allow(non_upper_case_globals)]
        const OsNonConforming = 0x100;
        #[allow(non_upper_case_globals)]
        const Group = 0x200;
        #[allow(non_upper_case_globals)]
        const Tls = 0x400;
        #[allow(non_upper_case_globals)]
        const MaskOs = 0x0ff00000;
        #[allow(non_upper_case_globals)]
        const MaskProc = 0xf0000000;
        #[allow(non_upper_case_globals)]
        const Ordered = 0x40000000;
        #[allow(non_upper_case_globals)]
        const Exclude = 0x80000000;
    }
}

impl TryFrom<u32> for SectionHeaderFlags {
    type Error = ElfParseError;
    fn try_from(value: u32) -> Result<Self> {
        Ok(Self::from_bits(value).ok_or(ElfParseError::InvalidSectionHeaderFlags)?)
    }
}

#[derive(Debug)]
pub struct SectionHeader {
    sh_name: u32,
    sh_type: SectionHeaderType,
    sh_flags: SectionHeaderFlags,
    sh_addr: u32,
    sh_offset: u32,
    sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32,
}

impl SectionHeader {
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self> {
        Self::try_from(bytes.as_ref())
    }
}

impl TryFrom<&[u8]> for SectionHeader {
    type Error = ElfParseError;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < mem::size_of::<Self>() {
            return Err(ElfParseError::InvalidSectionHeader);
        }

        let new = Self {
            sh_name: slice_to_u32(&value[0x0..0x4]),
            sh_type: SectionHeaderType::try_from(slice_to_u32(&value[0x4..0x8]))?,
            sh_flags: SectionHeaderFlags::try_from(slice_to_u32(&value[0x8..0xC]))?,
            sh_addr: slice_to_u32(&value[0xC..0x10]),
            sh_offset: slice_to_u32(&value[0x10..0x14]),
            sh_size: slice_to_u32(&value[0x14..0x18]),
            sh_link: slice_to_u32(&value[0x18..0x1C]),
            sh_info: slice_to_u32(&value[0x1C..0x20]),
            sh_addralign: slice_to_u32(&value[0x20..0x24]),
            sh_entsize: slice_to_u32(&value[0x24..0x28]),
        };
        if !new.sh_addralign.is_power_of_two() && new.sh_addralign != 0 {
            return Err(ElfParseError::InvalidSectionAlignment);
        }

        Ok(new)
    }
}

impl core::fmt::Display for SectionHeader {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(write!(fmt, "name: {:04x}, addr: {:08X}, offset: {:08X}, size: {:08X}, type: {:?}, flags: {:8?}, link: {:2}, info: {}, align: {}, entsize: {:02X}",
                  self.sh_name,
                  self.sh_addr,
                  self.sh_offset,
                  self.sh_size,
                  self.sh_type,
                  self.sh_flags,
                  self.sh_link,
                  self.sh_info,
                  self.sh_addralign,
                  self.sh_entsize
        )?)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Symbol {
    name: u32,
    value: u32,
    size: u32,
    info: SymbolInfo,
    visibility: SymbolVisibility,
    section_index: u16,
}

impl TryFrom<&[u8]> for Symbol {
    type Error = ElfParseError;
    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < mem::size_of::<Self>() {
            return Err(ElfParseError::InvalidSymbol);
        }

        Ok(Self {
            name: slice_to_u32(&value[0..0x4]),
            value: slice_to_u32(&value[0x4..0x8]),
            size: slice_to_u32(&value[0x8..0xC]),
            info: SymbolInfo::try_from(value[0xC])?,
            visibility: SymbolVisibility::try_from(value[0xD])?,
            section_index: slice_to_u16(&value[0xE..0x10]),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, is_enum_variant)]
pub enum SymbolType {
    NoType,
    Object,
    Func,
    Section,
    File,
    Common,
    Tls,
    Num,
    Loos,
    GnuIfunc,
    Hios,
    LoProc,
    HiProc,
}

impl TryFrom<u8> for SymbolType {
    type Error = ElfParseError;
    fn try_from(value: u8) -> Result<Self> {
        use SymbolType::*;
        Ok(match value {
            0 => NoType,
            1 => Object,
            2 => Func,
            3 => Section,
            4 => File,
            5 => Common,
            6 => Tls,
            7 => Num,
            10 => GnuIfunc,
            // 10 => Loos,
            12 => Hios,
            13 => LoProc,
            15 => HiProc,
            _ => return Err(ElfParseError::InvalidSymbolType),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, is_enum_variant)]
pub enum SymbolBinding {
    Local,
    Global,
    Weak,
    Num,
    Loos,
    GnuUnique,
    Hios,
    LoProc,
    HiProc,
}

impl TryFrom<u8> for SymbolBinding {
    type Error = ElfParseError;
    fn try_from(value: u8) -> Result<Self> {
        use SymbolBinding::*;
        Ok(match value {
            0 => Local,
            1 => Global,
            2 => Weak,
            3 => Num,
            10 => GnuUnique,
            // 10 => Loos,
            12 => Hios,
            13 => LoProc,
            15 => HiProc,
            _ => return Err(ElfParseError::InvalidSymbolBinding),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SymbolInfo {
    s_type: SymbolType,
    s_binding: SymbolBinding,
}

impl SymbolInfo {
    pub fn get_type(&self) -> SymbolType {
        self.s_type
    }

    pub fn get_binding(&self) -> SymbolBinding {
        self.s_binding
    }
}

impl TryFrom<u8> for SymbolInfo {
    type Error = ElfParseError;
    fn try_from(value: u8) -> Result<Self> {
        let bits_type = value & 0xf;
        let bits_binding = value >> 4;

        Ok(Self { s_type: bits_type.try_into()?, s_binding: bits_binding.try_into()? })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, is_enum_variant)]
pub enum SymbolVisibility {
    Default,
    Internal,
    Hidden,
    Protected,
}

impl TryFrom<u8> for SymbolVisibility {
    type Error = ElfParseError;
    fn try_from(value: u8) -> Result<Self> {
        use SymbolVisibility::*;
        Ok(match value {
            0 => Default,
            1 => Internal,
            2 => Hidden,
            3 => Protected,
            _ => return Err(ElfParseError::InvalidSymbolVisibility),
        })
    }
}

pub struct Rel {
    offset: u32,
    info: u32,
}

impl TryFrom<&[u8]> for Rel {
    type Error = ElfParseError;
    #[rustfmt::skip]
    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < mem::size_of::<Self>() {
            return Err(ElfParseError::InvalidRel);
        }

        Ok(Self { offset: slice_to_u32(&value[0x0..0x4]),
                  info: slice_to_u32(&value[0x4..0x8]) })
    }
}

pub struct Rela {
    rel: Rel,
    addend: i32,
}

impl TryFrom<&[u8]> for Rela {
    type Error = ElfParseError;
    #[rustfmt::skip]
    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < mem::size_of::<Self>() {
            return Err(ElfParseError::InvalidRela);
        }

        Ok(Self { rel: Rel::try_from(&value[0x0..0x8])?,
                  addend: slice_to_u32(&value[0x8..0xC]) as i32 })
    }
}

// pub struct DynamicTag {
//     tag: DynamicTagType,

// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        use std::env;
        use std::fs::File;
        use std::io::Read;
        let args = env::args();

        for arg in args.skip(3) {
            println!("{}<", arg);
            let mut file = File::open(&arg).unwrap();
            let mut content = Vec::new();

            file.read_to_end(&mut content).unwrap();

            let header = ElfHeader::from_bytes(&content).unwrap();

            println!("{:#X?}", &header);
            use core::slice;
            let program_header_table: &[[u8; mem::size_of::<ProgramHeader>()]] = unsafe {
                slice::from_raw_parts(
                    &content[header.program_header_table_offset as usize] as *const u8 as *const _,
                    header.nbr_program_header as usize,
                )
            };

            let mut ph_table = Vec::new();

            println!("\nProgram header table:");
            for (index, program_header) in program_header_table.iter().enumerate() {
                let pheader = ProgramHeader::from_bytes(program_header as &[u8]).unwrap();
                println!("{}: {:#X?}", index, pheader);
                ph_table.push(pheader);
            }

            let section_header_table: &[[u8; mem::size_of::<SectionHeader>()]] = unsafe {
                slice::from_raw_parts(
                    &content[header.section_header_table_offset as usize] as *const u8 as *const _,
                    header.nbr_section_header as usize,
                )
            };

            let mut sh_table = Vec::new();

            println!("\nSection header table:");
            for (index, section_header) in section_header_table.iter().enumerate() {
                let sheader = SectionHeader::from_bytes(section_header as &[u8]).unwrap();
                println!("{:02}: {:?}", index, sheader);
                sh_table.push(sheader);
            }
        }
    }
}
