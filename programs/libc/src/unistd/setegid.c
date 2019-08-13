#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/// If gid is equal to the real group ID or the saved set-group-ID, or
/// if the process has appropriate privileges, setegid() shall set the
/// effective group ID of the calling process to gid; the real group
/// ID, saved set-group-ID, and any supplementary group IDs shall
/// remain unchanged.
/// 
/// The setegid() function shall not affect the supplementary group
/// list in any way.

int setegid(gid_t gid)
{
	int ret = _user_syscall(SETEGID, 1, gid);
	set_errno_and_return(ret);
}

