
#include <user_syscall.h>
#include <sys/socket.h>

extern int errno;

struct s_recv {
	int sockfd;
	void *buf;
	size_t len;
	int flags;
};

/*
 * Receive a message from a socket
 */
ssize_t recv(int sockfd, void *buf, size_t len, int flags)
{
	struct s_recv s = {sockfd, buf, len, flags};

	ssize_t ret = (ssize_t)_user_syscall(SOCKETCALL, 2, __RECV, &s);
	/*
	 * This call returns the number of bytes received, or -1 if an error occurred.
	 * In the event of an error, errno is set to indicate the error.
	 * When a stream socket peer has performed an orderly shutdown,
	 * the return value will be 0 (the traditional "end-of-file" return).
	 */
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return ret;
	}
}
