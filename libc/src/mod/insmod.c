#include <mod.h>
#include <errno.h>
#include <user_syscall.h>

/*
 * Insert a kernel module
 */
int insmod(const char *modname) {
	int ret = _user_syscall(INSMOD, 1, modname);
	/*
	 * On success: Return 0, on error, -1
	 * In case of error, 'errno' may be set to:
	 * EACCESS (not enought permissions)
	 * EINVAL (bad module name)
	 * ENOMEM (not enought memory)
	 * EFAULT (bad modname address)
	 */
	set_errno_and_return(ret);
}
