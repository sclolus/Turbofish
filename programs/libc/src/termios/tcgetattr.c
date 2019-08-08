#include <termios.h>
#include <errno.h>
#include <user_syscall.h>

int tcgetattr(int fildes, struct termios *termios_p) {
	int ret = _user_syscall(TCGETATTR, 2, fildes, termios_p);
	set_errno_and_return(ret);
}
