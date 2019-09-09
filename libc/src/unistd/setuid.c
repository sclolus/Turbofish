#include <ltrace.h>
#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/// If the process has appropriate privileges, setuid() shall set the
/// real user ID, effective user ID, and the saved set-user-ID of the
/// calling process to uid.
/// 
/// If the process does not have appropriate privileges, but uid is
/// equal to the real user ID or the saved set-user-ID, setuid() shall
/// set the effective user ID to uid; the real user ID and saved
/// set-user-ID shall remain unchanged.
/// 
/// The setuid() function shall not affect the supplementary group
/// list in any way.

int setuid(uid_t uid)
{
	TRACE
	int ret = _user_syscall(SETUID, 1, uid);
	set_errno_and_return(ret);
}
