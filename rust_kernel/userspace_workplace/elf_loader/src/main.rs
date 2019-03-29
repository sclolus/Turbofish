#![feature(try_from)]

#[macro_use]
extern crate bitflags;
use core::convert::TryFrom;
use core::mem;

#[derive(Debug)]
enum ElfParseError {
    BadMagic,
    InvalidHeader,
    InvalidEndian,
    InvalidFormat,
    InvalidAbi,
    InvalidObjectType,
    InvalidTargetArchitecture,
    InvalidSegmentType,
    InvalidProgramHeader,
    InvalidSegmentAlignment,
    InvalidSectionHeaderType,
    InvalidSectionAlignment,
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

#[derive(Debug)]
enum Endian {
    Little,
    Big,
}

impl TryFrom<u8> for Endian {
    type Error = ElfParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Endian::*;
        match value {
            1 => Ok(Little),
            2 => Ok(Big),
            _ => Err(ElfParseError::InvalidEndian),
        }
    }
}

#[derive(Debug)]
enum Format {
    Bit32,
    Bit64,
}

impl TryFrom<u8> for Format {
    type Error = ElfParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Format::*;
        match value {
            1 => Ok(Bit32),
            2 => Ok(Bit64),
            _ => Err(ElfParseError::InvalidFormat),
        }
    }
}

#[derive(Debug)]
enum Abi {
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

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

#[derive(Debug)]
enum ObjectType {
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

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use crate::ObjectType::*;
        println!("{:x}", value);
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

#[derive(Debug)]
enum Architecture {
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

    fn try_from(value: u16) -> Result<Self, Self::Error> {
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

struct ElfHeader {
    /// 32-bit 64-bit.
    format: Format,

    /// Endianness of this object file.
    endian: Endian,

    /// Identifies the target operating system ABI.
    target_abi: Abi,

    /// Further specifies the ABI.
    abi_version: u8,

    /// The object type of file.
    e_type: ObjectType,

    /// The target architecture of this object file.
    e_machine: Architecture,

    /// Version of ELF used. Probably 1.
    e_version: u32,

    /// The address of the entry point. i.e the address in memory where the programs starts executing.
    e_entry: u32,

    /// Offset of the start of the program header table.
    e_phoff: u32,

    /// Offset of the start of the section header table.
    e_shoff: u32,

    /// The interpretation of this field depends on the target architecture.
    e_flags: u32,

    /// The size of this header.
    e_ehsize: u16,

    /// Contains the size of a program header table entry.
    e_phentsize: u16,

    /// Contains the number of entries in the program header table.
    e_phnum: u16,

    /// Contains the size of a section header table.
    e_shentsize: u16,

    /// Contains the number of entries in the section header table.
    e_shnum: u16,

    /// Contains index of the section header table entry that contains the section names.
    e_shstrndx: u16,
}

impl ElfHeader {
    fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, ElfParseError> {
        Self::try_from(bytes.as_ref())
    }
}

impl TryFrom<&[u8]> for ElfHeader {
    type Error = ElfParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 52 {
            return Err(ElfParseError::InvalidHeader);
        }
        // check the magic
        if value[0..4] != [0x7F, 0x45, 0x4c, 0x46] {
            return Err(ElfParseError::BadMagic);
        }

        Ok(Self {
            format: Format::try_from(value[0x4])?,
            endian: Endian::try_from(value[0x5])?,
            target_abi: Abi::try_from(value[0x7])?,
            abi_version: value[0x8],
            e_type: ObjectType::try_from(u16::from_ne_bytes(TryFrom::try_from(&value[0x10..0x12])?))?,
            e_machine: Architecture::try_from(u16::from_ne_bytes(TryFrom::try_from(&value[0x12..0x14])?))?,
            e_version: u32::from_ne_bytes(TryFrom::try_from(&value[0x14..0x18])?),
            e_entry: u32::from_ne_bytes(TryFrom::try_from(&value[0x18..0x1C])?),
            e_phoff: u32::from_ne_bytes(TryFrom::try_from(&value[0x1C..0x20])?),
            e_shoff: u32::from_ne_bytes(TryFrom::try_from(&value[0x20..0x24])?),
            e_flags: u32::from_ne_bytes(TryFrom::try_from(&value[0x24..0x28])?),
            e_ehsize: u16::from_ne_bytes(TryFrom::try_from(&value[0x28..0x2A])?),
            e_phentsize: u16::from_ne_bytes(TryFrom::try_from(&value[0x2A..0x2C])?),
            e_phnum: u16::from_ne_bytes(TryFrom::try_from(&value[0x2C..0x2E])?),
            e_shentsize: u16::from_ne_bytes(TryFrom::try_from(&value[0x2E..0x30])?),
            e_shnum: u16::from_ne_bytes(TryFrom::try_from(&value[0x30..0x32])?),
            e_shstrndx: u16::from_ne_bytes(TryFrom::try_from(&value[0x32..0x34])?),
        })
    }
}

impl core::fmt::Debug for ElfHeader {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(write!(fmt, "Class: {:?}, Format: {:?}, version: {}, ABI: {:?}, abi_version: {}, type: {:?}, machine: {:?}, entry_point_address: {:08x}, start_ph: {}, start_sh: {}, flags: {}, header_size: {}, size_of_ph: {}, nbr_ph: {}, size_sh: {}, nbr_sh: {}, section header string table index: {}",
                  self.format,
                  self.endian,
                  self.e_version,
                  self.target_abi,
                  self.abi_version,
                  self.e_type,
                  self.e_machine,
                  self.e_entry,
                  self.e_phoff,
                  self.e_shoff,
                  self.e_flags,
                  self.e_ehsize,
                  self.e_phentsize,
                  self.e_phnum,
                  self.e_shentsize,
                  self.e_shnum,
                  self.e_shstrndx,

        )?)
    }
}

#[derive(Debug)]
enum SegmentType {
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
}

bitflags! {
    struct ProgramHeaderFlags: u32 {
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
// TODO should be a TryFrom.
impl From<u32> for ProgramHeaderFlags {
    fn from(value: u32) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl TryFrom<u32> for SegmentType {
    type Error = ElfParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
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
            _ => return Err(ElfParseError::InvalidSegmentType),
        })
    }
}

struct ProgramHeader {
    p_type: SegmentType,
    p_offset: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filez: u32,
    p_memsz: u32,
    p_flags: ProgramHeaderFlags,
    p_align: u32,
}

impl ProgramHeader {
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, ElfParseError> {
        Self::try_from(bytes.as_ref())
    }
}

impl TryFrom<&[u8]> for ProgramHeader {
    type Error = ElfParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 32 {
            return Err(ElfParseError::InvalidProgramHeader);
        }

        let new = Self {
            p_type: SegmentType::try_from(u32::from_ne_bytes(copy_to_array(&value[0x0..0x4])))?,
            p_offset: (u32::from_ne_bytes(copy_to_array(&value[0x4..0x8]))),
            p_vaddr: (u32::from_ne_bytes(copy_to_array(&value[0x8..0x0C]))),
            p_paddr: (u32::from_ne_bytes(copy_to_array(&value[0x0C..0x10]))),
            p_filez: (u32::from_ne_bytes(copy_to_array(&value[0x10..0x14]))),
            p_memsz: (u32::from_ne_bytes(copy_to_array(&value[0x14..0x18]))),
            p_flags: (ProgramHeaderFlags::from(u32::from_ne_bytes(copy_to_array(&value[0x18..0x1C])))),
            p_align: (u32::from_ne_bytes(copy_to_array(&value[0x1C..0x20]))),
        };

        // if new.p_align > 1 && (new.p_vaddr != new.p_offset % new.p_align || !new.p_align.is_power_of_two()) {
        //     //TODO Loadable  process  segments must have congruent values for
        //     // p_vaddr and p_offset, modulo the page size.
        //     return Err(ElfParseError::InvalidSegmentAlignment);
        // }
        Ok(new)
    }
}

impl core::fmt::Debug for ProgramHeader {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(write!(fmt, "type: {:?}, offset: {:08X}, vaddr: {:08X}, paddr: {:08X}, filez: {:08x}, memsz: {:08X}, flags: {:?}, align: {:02X}",
                  self.p_type,
                  self.p_offset,
                  self.p_vaddr,
                  self.p_paddr,
                  self.p_filez,
                  self.p_memsz,
                  self.p_flags,
                  self.p_align,
        )?)
    }
}

#[derive(Debug)]
enum SectionHeaderType {
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

    fn try_from(value: u32) -> Result<Self, Self::Error> {
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
            0x6ffffffa => SunwMove,
            0x6ffffffb => SunwComdat,
            0x6ffffffc => SunwSyminfo,
            0x6ffffffd => GnuVerdef,
            0x6ffffffe => GnuVerneed,
            0x6fffffff => GnuVersym,
            0x6fffffff => Hisunw,
            0x6fffffff => Hios,
            0x70000000 => LoProc,
            0x7fffffff => HiProc,
            0x80000000 => LoUser,
            0x8fffffff => HiUser,
            _ => return Err(ElfParseError::InvalidSectionHeaderType),
        })
    }
}

bitflags! {
    struct SectionHeaderFlags: u32 {
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

// TODO should be a TryFrom.
impl From<u32> for SectionHeaderFlags {
    fn from(value: u32) -> Self {
        unsafe { mem::transmute(value) }
    }
}

struct SectionHeader {
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
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, ElfParseError> {
        Self::try_from(bytes.as_ref())
    }
}

impl TryFrom<&[u8]> for SectionHeader {
    type Error = ElfParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        fn slice_to_u32(slice: &[u8]) -> u32 {
            u32::from_ne_bytes(copy_to_array(&slice[0x0..0x4]))
        }

        let new = Self {
            sh_name: slice_to_u32(&value[0x0..0x4]),
            sh_type: SectionHeaderType::try_from(slice_to_u32(&value[0x4..0x8]))?,
            sh_flags: SectionHeaderFlags::from(slice_to_u32(&value[0x8..0xC])),
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

impl core::fmt::Debug for SectionHeader {
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

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::Read;
    let args = env::args();

    for arg in args.skip(1) {
        let mut file = File::open(&arg).unwrap();
        let mut content = Vec::new();

        file.read_to_end(&mut content).unwrap();

        let header = ElfHeader::from_bytes(&content).unwrap();

        println!("{:?}", header);
        use core::slice;
        let program_header_table: &[[u8; mem::size_of::<ProgramHeader>()]] = unsafe {
            slice::from_raw_parts(&content[header.e_phoff as usize] as *const u8 as *const _, header.e_phnum as usize)
        };

        let mut ph_table = Vec::new();

        println!("\nProgram header table:");
        for (index, program_header) in program_header_table.iter().enumerate() {
            let pheader = ProgramHeader::from_bytes(program_header as &[u8]).unwrap();
            println!("{}: {:?}", index, pheader);
            ph_table.push(pheader);
        }

        let section_header_table: &[[u8; mem::size_of::<SectionHeader>()]] = unsafe {
            slice::from_raw_parts(&content[header.e_shoff as usize] as *const u8 as *const _, header.e_shnum as usize)
        };

        let mut sh_table = Vec::new();

        println!("\nSection header table:");
        for (index, section_header) in section_header_table.iter().enumerate() {
            let sheader = SectionHeader::from_bytes(section_header as &[u8]).unwrap();
            // println!("{:02}: {:?}", index, sheader);
            sh_table.push(sheader);
        }
    }
}
