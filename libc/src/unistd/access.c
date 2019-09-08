#include <ltrace.h>
#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/// The access() function shall check the file named by the pathname
/// pointed to by the path argument for accessibility according to the
/// bit pattern contained in amode. The checks for accessibility
/// (including directory permissions checked during pathname
/// resolution) shall be performed using the real user ID in place of
/// the effective user ID and the real group ID in place of the
/// effective group ID.
///
/// The value of amode is either the bitwise-inclusive OR of the
/// access permissions to be checked (R_OK, W_OK, X_OK) or the
/// existence test (F_OK).
///
/// If any access permissions are checked, each shall be checked
/// individually, as described in XBD File Access Permissions, except
/// that where that description refers to execute permission for a
/// process with appropriate privileges, an implementation may
/// indicate success for X_OK even if execute permission is not
/// granted to any user.
int access(const char *path, int amode)
{
	TRACE
	int ret = _user_syscall(ACCESS, 2, path, amode);
	set_errno_and_return(ret);
}


