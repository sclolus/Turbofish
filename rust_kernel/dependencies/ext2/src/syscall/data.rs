//! contains the types and struct used by the syscall
//! we should finally put that on the libc
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use bitflags::bitflags;

//TODO: trash that
bitflags! {
    /// Flags for open syscall
    pub struct OpenFlags : u32 {
        /// Open for execute only (non-directory files). The result is
        /// unspecified if this flag is applied to a directory.
        const O_EXEC = 1 << 0;

        /// Open for reading only.
        const O_RDONLY = 1 << 1;

        /// Open for reading and writing. The result is undefined if
        /// this flag is applied to a FIFO.
        const O_RDWR = 1 << 2;

        /// Open directory for search only. The result is unspecified
        /// if this flag is applied to a non-directory file.
        const O_SEARCH = 1 << 3;

        /// Open for writing only.
        const O_WRONLY = 1 << 4;

        /*Any combination of the following may be used: */

        /// If set, the file offset shall be set to the end of the
        /// file prior to each write.
        const O_APPEND = 1 << 5;

        /// If set, the FD_CLOEXEC flag for the new file descriptor
        /// shall be set.
        const O_CLOEXEC = 1 << 6;

        /// If the file exists, this flag has no effect except as
        /// noted under O_EXCL below. Otherwise, if O_DIRECTORY is not
        /// set the file shall be created as a regular file; the user
        /// ID of the file shall be set to the effective user ID of
        /// the process; the group ID of the file shall be set to the
        /// group ID of the file's parent directory or to the
        /// effective group ID of the process; and the access
        /// permission bits (see <sys/stat.h>) of the file mode shall
        /// be set to the value of the argument following the oflag
        /// argument taken as type mode_t modified as follows: a
        /// bitwise AND is performed on the file-mode bits and the
        /// corresponding bits in the complement of the process' file
        /// mode creation mask. Thus, all bits in the file mode whose
        /// corresponding bit in the file mode creation mask is set
        /// are cleared. When bits other than the file permission bits
        /// are set, the effect is unspecified. The argument following
        /// the oflag argument does not affect whether the file is
        /// open for reading, writing, or for both. Implementations
        /// shall provide a way to initialize the file's group ID to
        /// the group ID of the parent directory. Implementations may,
        /// but need not, provide an implementation-defined way to
        /// initialize the file's group ID to the effective group ID
        /// of the calling process.
        const O_CREAT = 1 << 7;

        /// If path resolves to a non-directory file, fail and set
        /// errno to [ENOTDIR].
        const O_DIRECTORY = 1 << 8;

        /// [SIO] [Option Start] Write I/O operations on the file
        /// descriptor shall complete as defined by synchronized I/O
        /// data integrity completion. [Option End]
        const O_DSYNC = 1 << 9;

        /// If O_CREAT and O_EXCL are set, open() shall fail if the
        /// file exists. The check for the existence of the file and
        /// the creation of the file if it does not exist shall be
        /// atomic with respect to other threads executing open()
        /// naming the same filename in the same directory with O_EXCL
        /// and O_CREAT set. If O_EXCL and O_CREAT are set, and path
        /// names a symbolic link, open() shall fail and set errno to
        /// [EEXIST], regardless of the contents of the symbolic
        /// link. If O_EXCL is set and O_CREAT is not set, the result
        /// is undefined.
        const O_EXCL = 1 << 10;

        /// If set and path identifies a terminal device, open() shall
        /// not cause the terminal device to become the controlling
        /// terminal for the process. If path does not identify a
        /// terminal device, O_NOCTTY shall be ignored.
        const O_NOCTTY = 1 << 11;

        /// If path names a symbolic link, fail and set errno to
        /// [ELOOP].
        const O_NOFOLLOW = 1 << 12;

        /// When opening a FIFO with O_RDONLY or O_WRONLY set:
        ///     If O_NONBLOCK is set, an open() for reading-only shall
        ///     return without delay. An open() for writing-only shall
        ///     return an error if no process currently has the file
        ///     open for reading.

        ///     If O_NONBLOCK is clear, an open() for reading-only
        ///     shall block the calling thread until a thread opens
        ///     the file for writing. An open() for writing-only shall
        ///     block the calling thread until a thread opens the file
        ///     for reading.

        /// When opening a block special or character special file
        /// that supports non-blocking opens:

        ///     If O_NONBLOCK is set, the open() function shall return
        ///     without blocking for the device to be ready or
        ///     available. Subsequent behavior of the device is
        ///     device-specific.

        ///     If O_NONBLOCK is clear, the open() function shall
        ///     block the calling thread until the device is ready or
        ///     available before returning.

        /// Otherwise, the O_NONBLOCK flag shall not cause an error,
        /// but it is unspecified whether the file status flags will
        /// include the O_NONBLOCK flag.
        const O_NONBLOCK = 1 << 13;

        /// [SIO] [Option Start] Read I/O operations on the file
        /// descriptor shall complete at the same level of integrity
        /// as specified by the O_DSYNC and O_SYNC flags. If both
        /// O_DSYNC and O_RSYNC are set in oflag, all I/O operations
        /// on the file descriptor shall complete as defined by
        /// synchronized I/O data integrity completion. If both O_SYNC
        /// and O_RSYNC are set in flags, all I/O operations on the
        /// file descriptor shall complete as defined by synchronized
        /// I/O file integrity completion. [Option End]
        const O_RSYNC = 1 << 14;

        /// [XSI|SIO] [Option Start] Write I/O operations on the file
        /// descriptor shall complete as defined by synchronized I/O
        /// file integrity completion. [Option End]
        const O_SYNC = 1 << 15;

        ///[XSI] [Option Start] The O_SYNC flag shall be supported for
        /// regular files, even if the Synchronized Input and Output
        /// option is not supported. [Option End]
        /// If the file exists and is a regular file, and the file is
        /// successfully opened O_RDWR or O_WRONLY, its length shall
        /// be truncated to 0, and the mode and owner shall be
        /// unchanged. It shall have no effect on FIFO special files
        /// or terminal device files. Its effect on other file types
        /// is implementation-defined. The result of using O_TRUNC
        /// without either O_RDWR or O_WRONLY is undefined.
        const O_TRUNC = 1 << 16;

        /// If path identifies a terminal device other than a
        /// pseudo-terminal, the device is not already open in any
        /// process, and either O_TTY_INIT is set in oflag or
        /// O_TTY_INIT has the value zero, open() shall set any
        /// non-standard termios structure terminal parameters to a
        /// state that provides conforming behavior; see XBD
        /// Parameters that Can be Set. It is unspecified whether
        /// O_TTY_INIT has any effect if the device is already open in
        /// any process. If path identifies the slave side of a
        /// pseudo-terminal that is not already open in any process,
        /// open() shall set any non-standard termios structure
        /// terminal parameters to a state that provides conforming
        /// behavior, regardless of whether O_TTY_INIT is set. If path
        /// does not identify a terminal device, O_TTY_INIT shall be
        /// ignored.
        const O_TTY_INIT = 1 << 17;

    }
}

/* Encoding of the file mode.  */
// #define	__S_IFMT	0170000	/* These bits determine file type.  */
// /* File types.  */
// #define	__S_IFDIR	0040000	/* Directory.  */
// #define	__S_IFCHR	0020000	/* Character device.  */
// #define	__S_IFBLK	0060000	/* Block device.  */
// #define	__S_IFREG	0100000	/* Regular file.  */
// #define	__S_IFIFO	0010000	/* FIFO.  */
// #define	__S_IFLNK	0120000	/* Symbolic link.  */
// #define	__S_IFSOCK	0140000	/* Socket.  */
// /* Protection bits.  */
// #define	__S_ISUID	04000	/* Set user ID on execution.  */
// #define	__S_ISGID	02000	/* Set group ID on execution.  */
// #define	__S_ISVTX	01000	/* Save swapped text after use (sticky).  */
// #define	__S_IREAD	0400	/* Read by owner.  */
// #define	__S_IWRITE	0200	/* Write by owner.  */
// #define	__S_IEXEC	0100	/* Execute by owner.  */
