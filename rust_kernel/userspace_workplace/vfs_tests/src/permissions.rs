use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    #[allow(snake_case)]
    pub struct FilePermissions: u32 {
        /// Read, write, execute/search by owner.
        const S_IRWXU = 0o700;

        /// Read permission, owner.
        const S_IRUSR = 0o400;

        /// Write permission, owner.
        const S_IWUSR = 0o200;

        /// Execute/search permission, owner.
        const S_IXUSR = 0o100;

        /// Read, write, execute/search by group.
        const S_IRWXG = 0o70;

        /// Read permission, group.
        const S_IRGRP = 0o40;

        /// Write permission, group.
        const S_IWGRP = 0o20;

        /// Execute/search permission, group.
        const S_IXGRP = 0o10;

        /// Read, write, execute/search by others.
        const S_IRWXO = 0o7;

        ///Read permission, others.
        const S_IROTH = 0o4;

        /// Write permission, others.
        const S_IWOTH = 0o2;

        /// Execute/search permission, others.
        const S_IXOTH = 0o1;

        /// Set-user-ID on execution.
        const S_ISUID = 0o4000;

        /// Set-group-ID on execution.
        const S_ISGID = 0o2000;

        /// On directories, restricted deletion flag.   [Option End]
        const S_ISVTX = 0o1000;

        /// Filetypes.

        /// Character special
        const S_IFCHR = 0o10000;

        /// FIFO special
        const S_IFIFO = 0o20000;

        /// Regular
        const S_IFREG = 0o40000;

        /// Directory
        const S_IFDIR = 0o100000;

        /// Symbolic link
        const S_IFLNK = 0o200000;

        /// Socket
        const S_IFSOCK = 0o40000;
    }
}

impl FilePermissions {
    pub unsafe fn from_u32(mode: u32) -> Self {
        use std::mem::transmute;

        transmute(mode)
    }

    pub fn is_character_device(&self) -> bool {
        self.contains(Self::S_IFCHR)
    }

    pub fn is_fifo(&self) -> bool {
        self.contains(Self::S_IFIFO)
    }

    pub fn is_regular(&self) -> bool {
        self.contains(Self::S_IFREG)
    }

    pub fn is_directory(&self) -> bool {
        self.contains(Self::S_IFDIR)
    }

    pub fn is_symlink(&self) -> bool {
        self.contains(Self::S_IFLNK)
    }

    pub fn is_socket(&self) -> bool {
        self.contains(Self::S_IFSOCK)
    }
}
