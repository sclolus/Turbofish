#include "user_syscall.h"

void exit_qemu(int status) {
	_user_syscall(EXIT_QEMU, 1, status);
}
