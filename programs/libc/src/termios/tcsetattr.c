#include <termios.h>
#include <errno.h>
#include <user_syscall.h>

int tcsetattr(int fildes, int optional_actions,
			  const struct termios *termios_p) {
	int ret = _user_syscall(TCSETATTR, 3, fildes, optional_actions, termios_p);
	set_errno_and_return(ret);
}
