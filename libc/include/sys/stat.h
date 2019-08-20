#ifndef STAT_H
# define STAT_H

#include <sys/types.h>
#include <time.h>

struct stat {
	dev_t st_dev;            // Device ID of device containing file. 
	ino_t st_ino;            // File serial number. 
	mode_t st_mode;          // Mode of file (see below). 
	nlink_t st_nlink;        // Number of hard links to the file. 
	uid_t st_uid;            // User ID of file. 
	gid_t st_gid;            // Group ID of file. 
	//[XSI][Option Start]
	dev_t st_rdev;           // Device ID (if file is character or block special). 
	//[Option End]
	off_t st_size;           // For regular files, the file size in bytes. 
	// For symbolic links, the length in bytes of the 
	// pathname contained in the symbolic link. 
	//[SHM][Option Start]
	// For a shared memory object, the length in bytes. 
	//[Option End]
	//[TYM][Option Start]
	// For a typed memory object, the length in bytes. 
	//[Option End]
	// For other file types, the use of this field is 
	// unspecified. 
	struct timespec st_atim; // Last data access timestamp. 
	struct timespec st_mtim; // Last data modification timestamp. 
	struct timespec st_ctim; // Last file status change timestamp. 
	//[XSI][Option Start]
	blksize_t st_blksize;    // A file system-specific preferred I/O block size 
	// for this object. In some file system types, this 
	// may vary from file to file. 
	blkcnt_t st_blocks;      // Number of blocks allocated for this object. 
	//[Option End]

};

//TODO: Dash need that to compile, check that
struct stat64 {
	dev_t st_dev;            // Device ID of device containing file. 
	ino_t st_ino;            // File serial number. 
	mode_t st_mode;          // Mode of file (see below). 
	nlink_t st_nlink;        // Number of hard links to the file. 
	uid_t st_uid;            // User ID of file. 
	gid_t st_gid;            // Group ID of file. 
	//[XSI][Option Start]
	dev_t st_rdev;           // Device ID (if file is character or block special). 
	//[Option End]
	off_t st_size;           // For regular files, the file size in bytes. 
	// For symbolic links, the length in bytes of the 
	// pathname contained in the symbolic link. 
	//[SHM][Option Start]
	// For a shared memory object, the length in bytes. 
	//[Option End]
	//[TYM][Option Start]
	// For a typed memory object, the length in bytes. 
	//[Option End]
	// For other file types, the use of this field is 
	// unspecified. 
	struct timespec st_atim; // Last data access timestamp. 
	struct timespec st_mtim; // Last data modification timestamp. 
	struct timespec st_ctim; // Last file status change timestamp. 
	//[XSI][Option Start]
	blksize_t st_blksize;    // A file system-specific preferred I/O block size 
	// for this object. In some file system types, this 
	// may vary from file to file. 
	blkcnt_t st_blocks;      // Number of blocks allocated for this object. 
	//[Option End]
};

#define st_atime st_atim.tv_sec      /* Backward compatibility */
#define st_mtime st_mtim.tv_sec
#define st_ctime st_ctim.tv_sec
//The st_ino and st_dev fields taken together uniquely identify the file within the system.

//The <sys/stat.h> header shall define the [XSI] [Option Start] blkcnt_t, blksize_t, [Option End] dev_t, ino_t, mode_t, nlink_t, uid_t, gid_t, off_t, and time_t types as described in <sys/types.h>.

//The <sys/stat.h> header shall define the timespec structure as described in <time.h>. Times shall be given in seconds since the Epoch.

//Which structure members have meaningful values depends on the type of file. For further information, see the descriptions of fstat(), lstat(), and stat() in the System Interfaces volume of POSIX.1-2017.

//For compatibility with earlier versions of this standard, the st_atime macro shall be defined with the value st_atim.tv_sec. Similarly, st_ctime and st_mtime shall be defined as macros with the values st_ctim.tv_sec and st_mtim.tv_sec, respectively.

//The <sys/stat.h> header shall define the following symbolic constants for the file types encoded in type mode_t. The values shall be suitable for use in #if preprocessing directives:

#define S_IFMT  00170000
#define S_IFSOCK 0140000
#define S_IFLNK	 0120000
#define S_IFREG  0100000
#define S_IFBLK  0060000
#define S_IFDIR  0040000
#define S_IFCHR  0020000
#define S_IFIFO  0010000
#define S_ISUID  0004000
#define S_ISGID  0002000
#define S_ISVTX  0001000

#define S_IRWXU 00700
#define S_IRUSR 00400
#define S_IWUSR 00200
#define S_IXUSR 00100

#define S_IRWXG 00070
#define S_IRGRP 00040
#define S_IWGRP 00020
#define S_IXGRP 00010

#define S_IRWXO 00007
#define S_IROTH 00004
#define S_IWOTH 00002
#define S_IXOTH 00001
//S_IFMT
//   //  [XSI] [Option Start] Type of file.
//S_IFBLK
//    Block special.
//S_IFCHR
//    Character special.
//S_IFIFO
//    FIFO special.
//S_IFREG
//    Regular.
//S_IFDIR
//    Directory.
//S_IFLNK
//    Symbolic link.
//S_IFSOCK
//    Socket.
  // [Option End]

//The <sys/stat.h> header shall define the following symbolic constants for the file mode bits encoded in type mode_t, with the indicated numeric values. These macros shall expand to an expression which has a type that allows them to be used, either singly or OR'ed together, as the third argument to open() without the need for a mode_t cast. The values shall be suitable for use in #if preprocessing directives.


//On directories, restricted deletion flag.   [Option End]

//The following macros shall be provided to test whether a file is of the specified type. The value m supplied to the macros is the value of st_mode from a stat structure. The macro shall evaluate to a non-zero value if the test is true; 0 if the test is false.

#define S_ISLNK(m)	(((m) & S_IFMT) == S_IFLNK)
#define S_ISREG(m)	(((m) & S_IFMT) == S_IFREG)
#define S_ISDIR(m)	(((m) & S_IFMT) == S_IFDIR)
#define S_ISCHR(m)	(((m) & S_IFMT) == S_IFCHR)
#define S_ISBLK(m)	(((m) & S_IFMT) == S_IFBLK)
#define S_ISFIFO(m)	(((m) & S_IFMT) == S_IFIFO)
#define S_ISSOCK(m)	(((m) & S_IFMT) == S_IFSOCK)
//S_ISBLK(m)
//    Test for a block special file.
//S_ISCHR(m)
//    Test for a character special file.
//S_ISDIR(m)
//    Test for a directory.
//S_ISFIFO(m)
//    Test for a pipe or FIFO special file.
//S_ISREG(m)
//    Test for a regular file.
//S_ISLNK(m)
//    Test for a symbolic link.
//S_ISSOCK(m)
//    Test for a socket.

//The implementation may implement message queues, semaphores, or shared memory objects as distinct file types. The following macros shall be provided to test whether a file is of the specified type. The value of the buf argument supplied to the macros is a pointer to a stat structure. The macro shall evaluate to a non-zero value if the specified object is implemented as a distinct file type and the specified file type is contained in the stat structure referenced by buf. Otherwise, the macro shall evaluate to zero.

//S_TYPEISMQ(buf)
//    Test for a message queue.
//S_TYPEISSEM(buf)
//    Test for a semaphore.
//S_TYPEISSHM(buf)
//    Test for a shared memory object.

//[TYM] [Option Start] The implementation may implement typed memory objects as distinct file types, and the following macro shall test whether a file is of the specified type. The value of the buf argument supplied to the macros is a pointer to a stat structure. The macro shall evaluate to a non-zero value if the specified object is implemented as a distinct file type and the specified file type is contained in the stat structure referenced by buf. Otherwise, the macro shall evaluate to zero.

//S_TYPEISTMO(buf)
//    Test macro for a typed memory object.

//[Option End]

//The <sys/stat.h> header shall define the following symbolic constants as distinct integer values outside of the range [0,999999999], for use with the futimens() and utimensat() functions: UTIME_NOW UTIME_OMIT

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.
int chmod(const char *, mode_t);
int fchmod(int, mode_t);
int fchmodat(int, const char *, mode_t, int);
int fstat(int, struct stat *);
int fstatat(int, const char *restrict, struct stat *restrict, int);
int futimens(int, const struct timespec [2]);
int lstat(const char *restrict, struct stat *restrict);
int mkdir(const char *, mode_t);
int mkdirat(int, const char *, mode_t);
int mkfifo(const char *, mode_t);
int mkfifoat(int, const char *, mode_t);
//[XSI][Option Start]
int mknod(const char *, mode_t, dev_t);
int mknodat(int, const char *, mode_t, dev_t);
//[Option End]
int stat(const char *restrict, struct stat *restrict);
mode_t umask(mode_t);
int utimensat(int, const char *, const struct timespec [2], int);

#endif
