
#include <user_syscall.h>
#include <errno.h>

extern int errno;

/*
 * Execute program
 */
int execve(const char *filename, char *const argv[], char *const envp[])
{
	int ret = _user_syscall(EXECVE, 3, filename, argv, envp);
	/*
	 * On success, execve() does not return, on error -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
