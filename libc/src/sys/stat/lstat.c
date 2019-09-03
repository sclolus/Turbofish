/*
 * lstat - get file status
 */

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

#include <sys/stat.h>

#include <user_syscall.h>
#include <errno.h>

#include <custom.h>

/*
 * lstat() is identical to stat(), except that if pathname is a symbolic link,
 * then it returns information about the link itself, not the file that it refers to.
 */
int lstat(const char *restrict pathname, struct stat *restrict stat)
{
	int ret = _user_syscall(LSTAT, 2, pathname, stat);

	/*
	 * On success, zero is returned.  On error, -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}

/*
 * Since our off_t structure is encoded in 64 bits, the lstat64() function is the same as lstat()
 */
int lstat64(const char *restrict pathname, struct stat64 *restrict stat)
{
	return lstat(pathname, (struct stat *restrict)stat);
}
