#include <sys/stat.h>
#include <errno.h>

// The stat() function shall obtain information about the named file
// and write it to the area pointed to by the buf argument. The path
// argument points to a pathname naming a file. Read, write, or
// execute permission of the named file is not required. An
// implementation that provides additional or alternate file access
// control mechanisms may, under implementation-defined conditions,
// cause stat() to fail. In particular, the system may deny the
// existence of the file specified by path.

// If the named file is a symbolic link, the stat() function shall
// continue pathname resolution using the contents of the symbolic
// link, and shall return information pertaining to the resulting file
// if the file exists.

// The buf argument is a pointer to a stat structure, as defined in
// the <sys/stat.h> header, into which information is placed
// concerning the file.

// The stat() function shall update any time-related fields (as
// described in XBD File Times Update), before writing into the stat
// structure.

// [SHM] [Option Start] If the named file is a shared memory object,
// the implementation shall update in the stat structure pointed to by
// the buf argument the st_uid, st_gid, st_size, and st_mode fields,
// and only the S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, and
// S_IWOTH file permission bits need be valid. The implementation may
// update other fields and flags. [Option End]

// [TYM] [Option Start] If the named file is a typed memory object,
// the implementation shall update in the stat structure pointed to by
// the buf argument the st_uid, st_gid, st_size, and st_mode fields,
// and only the S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, and
// S_IWOTH file permission bits need be valid. The implementation may
// update other fields and flags. [Option End]

// For all other file types defined in this volume of POSIX.1-2017,
// the structure members st_mode, st_ino, st_dev, st_uid, st_gid,
// st_atim, st_ctim, and st_mtim shall have meaningful values and the
// value of the member st_nlink shall be set to the number of links to
// the file.

// The lstat() function shall be equivalent to stat(), except when
// path refers to a symbolic link. In that case lstat() shall return
// information about the link, while stat() shall return information
// about the file the link references.

// For symbolic links, the st_mode member shall contain meaningful
// information when used with the file type macros. The file mode bits
// in st_mode are unspecified. The structure members st_ino, st_dev,
// st_uid, st_gid, st_atim, st_ctim, and st_mtim shall have meaningful
// values and the value of the st_nlink member shall be set to the
// number of (hard) links to the symbolic link. The value of the
// st_size member shall be set to the length of the pathname contained
// in the symbolic link not including any terminating null byte.

// The fstatat() function shall be equivalent to the stat() or lstat()
// function, depending on the value of flag (see below), except in the
// case where path specifies a relative path. In this case the status
// shall be retrieved from a file relative to the directory associated
// with the file descriptor fd instead of the current working
// directory. If the access mode of the open file description
// associated with the file descriptor is not O_SEARCH, the function
// shall check whether directory searches are permitted using the
// current permissions of the directory underlying the file
// descriptor. If the access mode is O_SEARCH, the function shall not
// perform the check.

// Values for flag are constructed by a bitwise-inclusive OR of flags
// from the following list, defined in <fcntl.h>:

// AT_SYMLINK_NOFOLLOW If path names a symbolic link, the status of
//     the symbolic link is returned.

// If fstatat() is passed the special value AT_FDCWD in the fd
// parameter, the current working directory shall be used and the
// behavior shall be identical to a call to stat() or lstat()
// respectively, depending on whether or not the AT_SYMLINK_NOFOLLOW
// bit is set in flag.

#warning NOT IMPLEMENTED
#include <custom.h>

int stat(const char *restrict path, struct stat *restrict buf)
{
	DUMMY
	(void)path;
	(void)buf;
	errno = ENOSYS;
	return -1;
}

int stat64(const char *restrict path, struct stat *restrict buf)
{
	DUMMY
	(void)path;
	(void)buf;
	errno = ENOSYS;
	return -1;
}
