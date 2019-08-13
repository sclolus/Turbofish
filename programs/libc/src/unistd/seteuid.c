#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/// If uid is equal to the real user ID or the saved set-user-ID, or
/// if the process has appropriate privileges, seteuid() shall set the
/// effective user ID of the calling process to uid; the real user ID
/// and saved set-user-ID shall remain unchanged.
/// 
/// The seteuid() function shall not affect the supplementary group
/// list in any way.

int seteuid(uid_t uid)
{
	int ret = _user_syscall(SETEUID, 1, uid);
	set_errno_and_return(ret);
}

