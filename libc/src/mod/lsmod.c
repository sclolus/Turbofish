#include <mod.h>
#include <errno.h>
#include <user_syscall.h>

/*
 * List all loaded kernel module
 */
int lsmod() {
	int ret = _user_syscall(LSMOD, 0);
	/*
	 * On success: Return 0, on error, -1
	 */
	set_errno_and_return(ret);
}
