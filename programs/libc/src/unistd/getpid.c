
#include "user_syscall.h"
#include "signal.h"

pid_t getpid(void) {
	return _user_syscall(GETPID, 0);
}
