#include <termios.h>
#include <errno.h>
#include <user_syscall.h>

/// The tcgetattr() function shall get the parameters associated with
/// the terminal referred to by fildes and store them in the termios
/// structure referenced by termios_p. The fildes argument is an open
/// file descriptor associated with a terminal.
///
/// The termios_p argument is a pointer to a termios structure.
///
/// The tcgetattr() operation is allowed from any process.
int tcgetattr(int fildes, struct termios *termios_p) {
	int ret = _user_syscall(TCGETATTR, 2, fildes, termios_p);
	set_errno_and_return(ret);
}
