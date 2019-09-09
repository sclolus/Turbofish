#include <ltrace.h>
#include <utime.h>
#include <errno.h>
#include <user_syscall.h>

/// The utime() function shall set the access and modification times
/// of the file named by the path argument.
///
/// If times is a null pointer, the access and modification times of
/// the file shall be set to the current time. The effective user ID
/// of the process shall match the owner of the file, or the process
/// has write permission to the file or has appropriate privileges, to
/// use utime() in this manner.
///
/// If times is not a null pointer, times shall be interpreted as a
/// pointer to a utimbuf structure and the access and modification
/// times shall be set to the values contained in the designated
/// structure. Only a process with the effective user ID equal to the
/// user ID of the file or a process with appropriate privileges may
/// use utime() this way.
///
/// The utimbuf structure is defined in the <utime.h> header. The
/// times in the structure utimbuf are measured in seconds since the
/// Epoch.
///
/// Upon successful completion, the utime() function shall mark the
/// last file status change timestamp for update; see <sys/stat.h>.

#warning NOT IMPLEMENTED IN KERNEL

#include <custom.h>

int utime(const char *path, const struct utimbuf *times)
{
	DUMMY_KERNEL
	TRACE
	int ret = _user_syscall(UTIME, 2, path, times);
	set_errno_and_return(ret);
}
