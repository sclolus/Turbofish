use core::fmt;

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct DirectoryEntryHeader {
    /// Inode
    /*0 	3 	4*/
    pub inode: u32,
    /// Total size of this entry (Including all subfields)
    /*4 	5 	2*/
    pub entry_size: u16,
    /// Name Length least-significant 8 bits
    /*6 	6 	1*/
    name_length: u8,
    /// Type indicator (only if the feature bit for "directory entries have file type byte" is set, else this is the most-significant 8 bits of the Name Length)
    /*7 	7 	1*/
    type_indicator: DirectoryEntryType,
    /// N 	Name characters
    /*8 	8+N-1*/
    filename: Filename,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u8)]
enum DirectoryEntryType {
    UnknownType,
    RegularFile,
    Directory,
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
    SymbolicLink,
}

impl DirectoryEntryHeader {
    pub fn get_filename(&self) -> &str {
        unsafe {
            let slice: &[u8] = core::slice::from_raw_parts(&self.filename.0 as *const u8, self.name_length as usize);
            core::str::from_utf8_unchecked(slice)
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Filename(pub [u8; 256]);

impl fmt::Debug for Filename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", "Filename")
    }
}
