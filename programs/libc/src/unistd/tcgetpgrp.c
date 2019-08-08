#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

// The tcgetpgrp() function shall return the value of the process
// group ID of the foreground process group associated with the
// terminal.

// If there is no foreground process group, tcgetpgrp() shall return a
// value greater than 1 that does not match the process group ID of
// any existing process group.

// The tcgetpgrp() function is allowed from a process that is a member
// of a background process group; however, the information may be
// subsequently changed by a process that is a member of a foreground
// process group.

pid_t tcgetpgrp(int fildes) {
	int ret = _user_syscall(TCGETPGRP, 1, fildes);
	set_errno_and_return(ret);
}
