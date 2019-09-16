#include <sys/stat.h>
#include <user_syscall.h>
#include <errno.h>

#include <custom.h>

#warning NOT IMPLEMENTED

int mknod(const char *path, mode_t mode, dev_t dev)
{
	DUMMY_KERNEL
	int ret = _user_syscall(MKNOD, 3, path, mode, dev);
	set_errno_and_return(ret);
}
