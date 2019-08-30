use bitflags::bitflags;
use libc_binding::{
    S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFREG, S_IFSOCK, S_IRGRP, S_IROTH, S_IRUSR, S_IRWXG,
    S_IRWXO, S_IRWXU, S_ISGID, S_ISUID, S_ISVTX, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH,
    S_IXUSR,
};
bitflags! {
    #[derive(Default)]
    #[allow(snake_case)]
    pub struct FilePermissions: u32 {
        /// Read, write, execute/search by owner.
        const S_IRWXU = S_IRWXU;

        /// Read permission, owner.
        const S_IRUSR = S_IRUSR;

        /// Write permission, owner.
        const S_IWUSR = S_IWUSR;

        /// Execute/search permission, owner.
        const S_IXUSR = S_IXUSR;

        /// Read, write, execute/search by group.
        const S_IRWXG = S_IRWXG;

        /// Read permission, group.
        const S_IRGRP = S_IRGRP;

        /// Write permission, group.
        const S_IWGRP = S_IWGRP;

        /// Execute/search permission, group.
        const S_IXGRP = S_IXGRP;

        /// Read, write, execute/search by others.
        const S_IRWXO = S_IRWXO;

        ///Read permission, others.
        const S_IROTH = S_IROTH;

        /// Write permission, others.
        const S_IWOTH = S_IWOTH;

        /// Execute/search permission, others.
        const S_IXOTH = S_IXOTH;

        /// Set-user-ID on execution.
        const S_ISUID = S_ISUID;

        /// Set-group-ID on execution.
        const S_ISGID = S_ISGID;

        /// On directories, restricted deletion flag.   [Option End]
        const S_ISVTX = S_ISVTX;

        /// Filetypes.

        /// Character special
        const S_IFCHR = S_IFCHR;

        /// FIFO special
        const S_IFIFO = S_IFIFO;

        /// Regular
        const S_IFREG = S_IFREG;

        /// Directory
        const S_IFDIR = S_IFDIR;

        /// Symbolic link
        const S_IFLNK = S_IFLNK;

        /// Socket
        const S_IFSOCK = S_IFSOCK;
    }
}

impl FilePermissions {
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
