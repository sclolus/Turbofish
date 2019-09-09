#include <ltrace.h>
#include <unistd.h>
#include <user_syscall.h>

void _exit(int status)
{
	TRACE
	/*
	 * The exit() function does not return.
	 */
	_user_syscall(EXIT, 1, status);
	while (1) {}
}
