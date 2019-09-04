#include <sys/stat.h>
#include <user_syscall.h>
#include <errno.h>

/// The mkdir() function shall create a new directory with name
/// path. The file permission bits of the new directory shall be
/// initialized from mode. These file permission bits of the mode
/// argument shall be modified by the process' file creation mask.
///
/// When bits in mode other than the file permission bits are set, the
/// meaning of these additional bits is implementation-defined.
///
/// The directory's user ID shall be set to the process' effective
/// user ID. The directory's group ID shall be set to the group ID of
/// the parent directory or to the effective group ID of the
/// process. Implementations shall provide a way to initialize the
/// directory's group ID to the group ID of the parent
/// directory. Implementations may, but need not, provide an
/// implementation-defined way to initialize the directory's group ID
/// to the effective group ID of the calling process.
///
/// The newly created directory shall be an empty directory.
///
/// If path names a symbolic link, mkdir() shall fail and set errno to
/// [EEXIST].
///
/// Upon successful completion, mkdir() shall mark for update the last
/// data access, last data modification, and last file status change
/// timestamps of the directory. Also, the last data modification and
/// last file status change timestamps of the directory that contains
/// the new entry shall be marked for update.
int mkdir(const char *path, mode_t mode)
{
	int ret = _user_syscall(MKDIR, 2, path, mode);
	set_errno_and_return(ret);
}

