#include <unistd.h>
#include <user_syscall.h>

pid_t getpgrp(void)
{
	return _user_syscall(GETPGRP, 0);
}
