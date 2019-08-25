#include <user_syscall.h>
#include <sys/socket.h>
#include <errno.h>

struct s_send {
	int sockfd;
	const void *buf;
	size_t len;
	int flags;
};

/*
 * Send a message on a socket
 */
ssize_t send(int sockfd, const void *buf, size_t len, int flags)
{
	struct s_send s = {sockfd, buf, len, flags};

	ssize_t ret = (ssize_t)_user_syscall(SOCKETCALL, 2, __SEND, &s);
	/*
	 * On success, this call returns the number of bytes sent.
	 * On error, -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
