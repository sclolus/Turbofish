
#include <sys/mman.h>
#include <user_syscall.h>
#include <errno.h>

int mprotect(void *addr, size_t length, int prot)
{
	int ret = _user_syscall(MPROTECT, 3, addr, length, prot);

	set_errno_and_return(ret);
}
