#include <ltrace.h>
#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/// The rmdir() function shall remove a directory whose name is given
/// by path. The directory shall be removed only if it is an empty
/// directory.
///
/// If the directory is the root directory or the current working
/// directory of any process, it is unspecified whether the function
/// succeeds, or whether it shall fail and set errno to [EBUSY].
///
/// If path names a symbolic link, then rmdir() shall fail and set
/// errno to [ENOTDIR].
///
/// If the path argument refers to a path whose final component is
/// either dot or dot-dot, rmdir() shall fail.
///
/// If the directory's link count becomes 0 and no process has the
/// directory open, the space occupied by the directory shall be freed
/// and the directory shall no longer be accessible. If one or more
/// processes have the directory open when the last link is removed,
/// the dot and dot-dot entries, if present, shall be removed before
/// rmdir() returns and no new entries may be created in the
/// directory, but the directory shall not be removed until all
/// references to the directory are closed.
///
/// If the directory is not an empty directory, rmdir() shall fail and
/// set errno to [EEXIST] or [ENOTEMPTY].
///
/// Upon successful completion, rmdir() shall mark for update the last
/// data modification and last file status change timestamps of the
/// parent directory.
int rmdir(const char *path)
{
	TRACE
	int ret = _user_syscall(RMDIR, 1, path);
	set_errno_and_return(ret);
}
