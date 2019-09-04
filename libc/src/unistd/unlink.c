#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

/// The unlink() function shall remove a link to a file. If path names
/// a symbolic link, unlink() shall remove the symbolic link named by
/// path and shall not affect any file or directory named by the
/// contents of the symbolic link. Otherwise, unlink() shall remove
/// the link named by the pathname pointed to by path and shall
/// decrement the link count of the file referenced by the link.
///
/// When the file's link count becomes 0 and no process has the file
/// open, the space occupied by the file shall be freed and the file
/// shall no longer be accessible. If one or more processes have the
/// file open when the last link is removed, the link shall be removed
/// before unlink() returns, but the removal of the file contents
/// shall be postponed until all references to the file are closed.
///
/// The path argument shall not name a directory unless the process
/// has appropriate privileges and the implementation supports using
/// unlink() on directories.
///
/// Upon successful completion, unlink() shall mark for update the
/// last data modification and last file status change timestamps of
/// the parent directory. Also, if the file's link count is not 0, the
/// last file status change timestamp of the file shall be marked for
/// update.
int unlink(const char *pathname)
{
	int ret = _user_syscall(UNLINK, 1, pathname);
	set_errno_and_return(ret);
}
