#include <ltrace.h>
#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/// If the process has appropriate privileges, setgid() shall set the
/// real group ID, effective group ID, and the saved set-group-ID of
/// the calling process to gid.
/// 
/// If the process does not have appropriate privileges, but gid is
/// equal to the real group ID or the saved set-group-ID, setgid()
/// shall set the effective group ID to gid; the real group ID and
/// saved set-group-ID shall remain unchanged.
/// 
/// The setgid() function shall not affect the supplementary group
/// list in any way.
/// 
/// Any supplementary group IDs of the calling process shall remain
/// unchanged.

int setgid(gid_t gid)
{
	TRACE
	int ret = _user_syscall(SETGID, 1, gid);
	set_errno_and_return(ret);
}
