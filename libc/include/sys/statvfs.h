#ifndef __SYS_STATVFS_H__
# define __SYS_STATVFS_H__

# include <sys/types.h>
//    The <sys/statvfs.h> header shall define the statvfs structure, which shall include at least the following members:
//
struct statvfs {
	unsigned long f_bsize   ;// File system block size.
	unsigned long f_frsize  ;// Fundamental file system block size.
	fsblkcnt_t    f_blocks  ;// Total number of blocks on file system in units of f_frsize.
	fsblkcnt_t    f_bfree   ;// Total number of free blocks.
	fsblkcnt_t    f_bavail  ;// Number of free blocks available to
	// non-privileged proces;s.
	fsfilcnt_t    f_files   ;// Total number of file serial numbers.
	fsfilcnt_t    f_ffree   ;// Total number of free file serial numbers.
	fsfilcnt_t    f_favail  ;// Number of file serial numbers available to
	// non-privileged proces;s.
	unsigned long f_fsid    ;// File system ID.
	unsigned long f_flag    ;// Bit mask of f_flag values.
	unsigned long f_namemax ;// Maximum filename length.
};
//
//    The <sys/statvfs.h> header shall define the fsblkcnt_t and fsfilcnt_t types as described in <sys/types.h>.
//
//    The <sys/statvfs.h> header shall define the following symbolic constants for the f_flag member:
//
//    ST_RDONLY
//        Read-only file system.
//    ST_NOSUID
//        Does not support the semantics of the ST_ISUID and ST_ISGID file mode bits.
//

# define ST_RDONLY 0x1
# define ST_NOSUID 0x2

//    The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.
//
int fstatvfs(int, struct statvfs *);
int statvfs(const char *restrict path, struct statvfs *restrict buf);

#endif
