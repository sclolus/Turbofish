#include <unistd.h>
#include <errno.h>
// The dup() function provides an alternative interface to the service
// provided by fcntl() using the F_DUPFD command. The call dup(fildes)
// shall be equivalent to:
 
// fcntl(fildes, F_DUPFD, 0);
 
// The dup2() function shall cause the file descriptor fildes2 to
// refer to the same open file description as the file descriptor
// fildes and to share any locks, and shall return fildes2. If fildes2
// is already a valid open file descriptor, it shall be closed first,
// unless fildes is equal to fildes2 in which case dup2() shall return
// fildes2 without closing it. If the close operation fails to close
// fildes2, dup2() shall return -1 without changing the open file
// description to which fildes2 refers. If fildes is not a valid file
// descriptor, dup2() shall return -1 and shall not close fildes2. If
// fildes2 is less than 0 or greater than or equal to {OPEN_MAX},
// dup2() shall return -1 with errno set to [EBADF].
 
// Upon successful completion, if fildes is not equal to fildes2, the
// FD_CLOEXEC flag associated with fildes2 shall be cleared. If fildes
// is equal to fildes2, the FD_CLOEXEC flag associated with fildes2
// shall not be changed.

#warning "NOT IMPLEMENTED"
#include <custom.h>

int dup2(int fildes, int fildes2)
{
	DUMMY
	(void)fildes;
	(void)fildes2;
	errno = ENOSYS;
	return -1;
}
