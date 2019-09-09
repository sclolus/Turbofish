#ifndef __FCNTL_H__
# define __FCNTL_H__

#include <sys/stat.h>
#include <unistd.h>

// The <fcntl.h> header shall define the following symbolic constant for use as the mask for file access modes. The value shall be suitable for use in #if preprocessing directives.
// The <fcntl.h> header shall define the following symbolic constants for use as the file access modes for open(), openat(), and fcntl(). The values shall be unique, except that O_EXEC and O_SEARCH may have equal values. The values shall be suitable for use in #if preprocessing directives.

#define O_RDONLY        00000001 // Open for reading only.
#define O_WRONLY        00000002 // Open for writing only.
#define O_RDWR          00000004 // Open for reading and writing.
#define O_EXEC          00000010 // Open for execute only (non-directory files). The result is unspecified if this flag is applied to a directory.
#define O_SEARCH        00000020 // Open directory for search only. The result is unspecified if this flag is applied to a non-directory file.
#define O_ACCMODE       00000040 // Mask for file access modes.

// The <fcntl.h> header shall define the following symbolic constants as file creation flags for use in the oflag value to open() and openat(). The values shall be bitwise-distinct and shall be suitable for use in #if preprocessing directives.

#define O_CREAT         00000100 // Create file if it does not exist.
#define O_EXCL          00000200 // Exclusive use flag.
#define O_NOCTTY        00000400 // Do not assign controlling terminal.

#define O_TRUNC         00001000 // Truncate flag.
#define O_APPEND        00002000 // Set append mode.
#define O_NONBLOCK      00004000 // Non-blocking mode.

#define O_DSYNC         00010000 // [SIO] [Option Start] Write according to synchronized I/O data integrity completion. [Option End]
#define O_RSYNC         00020000 // [SIO] [Option Start] Synchronized read I/O operations. [Option End]
#define O_SYNC          00040000 // Write according to synchronized I/O file integrity completion.

#define O_CLOEXEC       00100000 // The FD_CLOEXEC flag associated with the new descriptor shall be set to close the file descriptor upon execution of an exec family function.
#define O_DIRECTORY     00200000 // Fail if file is a non-directory file.
#define O_NOFOLLOW      00400000 // Do not follow symbolic links.

// The O_TTY_INIT flag can have the value zero and in this case it need not be bitwise-distinct from the other flags.
#define O_TTY_INIT	0        // Set the termios structure terminal parameters to a state that provides conforming behavior; see Parameters that Can be Set.

// The <fcntl.h> header shall define the following symbolic constants for the cmd argument used by fcntl(). The values shall be unique and shall be suitable for use in #if preprocessing directives.

// WARN: if you add or change a value, add it also in libc_binding/lib.rs
#define F_DUPFD 0 // Duplicate file descriptor.
#define F_DUPFD_CLOEXEC 1030 // Duplicate file descriptor with the close-on- exec flag FD_CLOEXEC set.
#define F_GETFD 1 // Get file descriptor flags.
#define F_SETFD 2 // Set file descriptor flags.
#define F_GETFL 3 // Get file status flags and file access modes.
#define F_SETFL 4 // Set file status flags.
#define F_GETLK 5 // Get record locking information.
#define F_SETLK 6 // Set record locking information.
#define F_SETLKW 7 // Set record locking information; wait if blocked.
#define F_SETOWN 8 // Set process or process group ID to receive SIGURG signals.
#define F_GETOWN 9 // Get process or process group ID to receive SIGURG signals.

//    The <fcntl.h> header shall define the following symbolic constant used for the fcntl() file descriptor flags, which shall be suitable for use in #if preprocessing directives.

#define FD_CLOEXEC	1
//        Close the file descriptor upon execution of an exec family function.

//    The <fcntl.h> header shall also define the following symbolic constants for the l_type argument used for record locking with fcntl(). The values shall be unique and shall be suitable for use in #if preprocessing directives.

#define F_RDLCK 0
//        Shared or read lock.
#define F_UNLCK 2
//        Unlock.
#define F_WRLCK 1
//        Exclusive or write lock.

//The <fcntl.h> header shall define the values used for l_whence, SEEK_SET, SEEK_CUR, and SEEK_END as described in <stdio.h>.
#include <stdio.h>

//    The <fcntl.h> header shall define the symbolic constants for file modes for use as values of mode_t as described in <sys/stat.h>.

//    The <fcntl.h> header shall define the following symbolic constant as a special value used in place of a file descriptor for the *at() functions which take a directory file descriptor as a parameter:

#define AT_FDCWD		-100
//        Use the current working directory to determine the target of relative file paths.

//    The <fcntl.h> header shall define the following symbolic constant as a value for the flag used by faccessat():

#define AT_EACCESS		0x200
//        Check access using effective user and group ID.

//    The <fcntl.h> header shall define the following symbolic constant as a value for the flag used by fstatat(), fchmodat(), fchownat(), and utimensat():

#define AT_SYMLINK_NOFOLLOW	0x100
//        Do not follow symbolic links.

//    The <fcntl.h> header shall define the following symbolic constant as a value for the flag used by linkat():

#define AT_SYMLINK_FOLLOW	0x400
//        Follow symbolic link.


//    The <fcntl.h> header shall define the following symbolic constant as a value for the flag used by unlinkat():

#define AT_REMOVEDIR		0x200
//        Remove directory instead of file.

//    [ADV] [Option Start] The <fcntl.h> header shall define the following symbolic constants for the advice argument used by posix_fadvise():

#define POSIX_FADV_DONTNEED 4
//        The application expects that it will not access the specified data in the near future.
#define POSIX_FADV_NOREUSE 5
//        The application expects to access the specified data once and then not reuse it thereafter.
#define POSIX_FADV_NORMAL	0
//        The application has no advice to give on its behavior with respect to the specified data. It is the default characteristic if no advice is given for an open file.
#define POSIX_FADV_RANDOM	1
//        The application expects to access the specified data in a random order.
#define POSIX_FADV_SEQUENTIAL	2
//        The application expects to access the specified data sequentially from lower offsets to higher offsets.
#define POSIX_FADV_WILLNEED	3
//        The application expects to access the specified data in the near future.

//    [Option End]

//    The <fcntl.h> header shall define the flock structure describing a file lock. It shall include the following members:

#include <sys/types.h>

struct flock {
	short  l_type   ; //Type of lock; F_RDLCK, F_WRLCK, F_UNLCK. 
	short  l_whence ; //Flag for starting offset. 
	off_t  l_start  ; //Relative offset in bytes. 
	off_t  l_len    ; //Size; if 0 then until EOF. 
	pid_t  l_pid    ; //Process ID of the process holding the lock; returned with F_GETLK. 
};

//The <fcntl.h> header shall define the mode_t, off_t, and pid_t types as described in <sys/types.h>.

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

int creat(const char *, mode_t);
int fcntl(int, int, ...);
int openat(int, const char *, int, ...);
//[ADV][Option Start]
int posix_fadvise(int, off_t, off_t, int);
int posix_fallocate(int, off_t, off_t);
//[Option End]

//Inclusion of the <fcntl.h> header may also make visible all symbols from <sys/stat.h> and <unistd.h>.
int open(const char *, int, ...);

#endif
