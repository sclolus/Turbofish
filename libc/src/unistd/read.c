#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

/*
 * Read from a file descriptor
 */
ssize_t read(int fd, void *buf, size_t count)
{
	ssize_t ret = _user_syscall(READ, 3, fd, buf, count);
	/*
	 * On success, the number of bytes read is returned (zero indicates end of file),
	 * and the file position is advanced by this number. It is not an error if this
	 * number is smaller than the number of bytes requested; this may happen for example
	 * because fewer bytes are actually available right now (maybe because we were close
	 * to end-of-file, or because we are reading from a pipe, or from a terminal), or
	 * because read() was interrupted by a signal. See also NOTES.
	 *
	 * On error, -1 is returned, and errno is set appropriately. In this case, it is left
	 * unspecified whether the file position (if any) changes
	 */

	set_errno_and_return(ret);
}
