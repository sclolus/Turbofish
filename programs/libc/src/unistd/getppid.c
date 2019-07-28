
#include <user_syscall.h>
#include <unistd.h>

pid_t        getppid(void) {
	return _user_syscall(GETPPID, 0);
}
