#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

// The pipe() function shall create a pipe and place two file
// descriptors, one each into the arguments fildes[0] and fildes[1],
// that refer to the open file descriptions for the read and write
// ends of the pipe. The file descriptors shall be allocated as
// described in File Descriptor Allocation. The O_NONBLOCK and
// FD_CLOEXEC flags shall be clear on both file descriptors. (The
// fcntl() function can be used to set both these flags.)

// Data can be written to the file descriptor fildes[1] and read from
// the file descriptor fildes[0]. A read on the file descriptor
// fildes[0] shall access data written to the file descriptor
// fildes[1] on a first-in-first-out basis. It is unspecified whether
// fildes[0] is also open for writing and whether fildes[1] is also
// open for reading.

// A process has the pipe open for reading (correspondingly writing)
// if it has a file descriptor open that refers to the read end,
// fildes[0] (write end, fildes[1]).

// The pipe's user ID shall be set to the effective user ID of the
// calling process.

// The pipe's group ID shall be set to the effective group ID of the
// calling process.

// Upon successful completion, pipe() shall mark for update the last
// data access, last data modification, and last file status change
// timestamps of the pipe.

int pipe(int fd[2])
{
	if (fd[0] < 0 || fd[1] < 0) {
		errno = EBADF;
		return -1;
	}

	int ret = _user_syscall(PIPE, 2, fd[0], fd[1]);

	set_errno_and_return(ret);
}
