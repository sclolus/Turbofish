
#include <user_syscall.h>
#include <stdlib.h>

/*
 * exit - cause normal process termination
 */
void exit(int status)
{
	/*
	 * The exit() function does not return.
	 */
	_user_syscall(EXIT, 1, status);
	while (1) {}
}
