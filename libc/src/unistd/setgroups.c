#include <ltrace.h>
#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

//// setgroups() sets the supplementary group IDs for the calling
//// process.  Appropriate privileges are required (see the
//// description of the EPERM error, below).  The size argument
//// specifies the number of supplementary group IDs in the buffer
//// pointed to by list.
int setgroups(size_t size, const gid_t *list)
{
	TRACE
	int ret = _user_syscall(SETGROUPS, 2, size, list);
	set_errno_and_return(ret);
}
