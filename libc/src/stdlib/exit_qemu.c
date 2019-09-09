#include <ltrace.h>
#include <user_syscall.h>

void exit_qemu(int status)
{
	TRACE
	_user_syscall(EXIT_QEMU, 1, status);
}
