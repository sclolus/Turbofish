#include <ltrace.h>
#include <user_syscall.h>
#include <sys/mman.h>
#include <errno.h>

int munmap(void *addr, size_t length)
{
	TRACE
	int ret = _user_syscall(MUNMAP, 2, addr, length);
	set_errno_and_return(ret);
}
