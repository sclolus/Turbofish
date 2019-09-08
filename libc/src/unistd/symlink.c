#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/// the symbolic link created, path1 is the string contained in the
/// symbolic link).
///
/// The string pointed to by path1 shall be treated only as a string
/// and shall not be validated as a pathname.
///
/// If the symlink() function fails for any reason other than [EIO],
/// any file named by path2 shall be unaffected.
///
/// If path2 names a symbolic link, symlink() shall fail and set errno
/// to [EEXIST].
///
/// The symbolic link's user ID shall be set to the process' effective
/// user ID. The symbolic link's group ID shall be set to the group ID
/// of the parent directory or to the effective group ID of the
/// process. Implementations shall provide a way to initialize the
/// symbolic link's group ID to the group ID of the parent
/// directory. Implementations may, but need not, provide an
/// implementation-defined way to initialize the symbolic link's group
/// ID to the effective group ID of the calling process.
///
/// The values of the file mode bits for the created symbolic link are
/// unspecified. All interfaces specified by POSIX.1-2017 shall behave
/// as if the contents of symbolic links can always be read, except
/// that the value of the file mode bits returned in the st_mode field
/// of the stat structure is unspecified.
///
/// Upon successful completion, symlink() shall mark for update the
/// last data access, last data modification, and last file status
/// change timestamps of the symbolic link. Also, the last data
/// modification and last file status change timestamps of the
/// directory that contains the new entry shall be marked for update.
int symlink(const char *path1, const char *path2) {
	int ret = _user_syscall(SYMLINK, 2, path1, path2);
	set_errno_and_return(ret);
}
