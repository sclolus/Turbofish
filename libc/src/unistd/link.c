#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/// The link() function shall create a new link (directory entry) for
/// the existing file, path1.
///
/// The path1 argument points to a pathname naming an existing
/// file. The path2 argument points to a pathname naming the new
/// directory entry to be created. The link() function shall
/// atomically create a new link for the existing file and the link
/// count of the file shall be incremented by one.
///
/// If path1 names a directory, link() shall fail unless the process
/// has appropriate privileges and the implementation supports using
/// link() on directories.
///
/// If path1 names a symbolic link, it is implementation-defined
/// whether link() follows the symbolic link, or creates a new link to
/// the symbolic link itself.
///
/// Upon successful completion, link() shall mark for update the last
/// file status change timestamp of the file. Also, the last data
/// modification and last file status change timestamps of the
/// directory that contains the new entry shall be marked for update.
///
/// If link() fails, no link shall be created and the link count of
/// the file shall remain unchanged.
///
/// The implementation may require that the calling process has
/// permission to access the existing file.
int link(const char *path1, const char *path2)
{
	int ret = _user_syscall(LINK, 2, path1, path2);
	set_errno_and_return(ret);
}
