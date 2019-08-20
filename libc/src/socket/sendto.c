#include <user_syscall.h>
#include <sys/socket.h>
#include <errno.h>

extern int errno;

struct s_sendto {
	int sockfd;
	const void *buf;
	size_t len;
	int flags;
	const struct sockaddr *dest_addr;
	socklen_t addrlen;
};

/*
 * Send a message on a socket
 */
ssize_t sendto(int sockfd, const void *buf, size_t len, int flags, const struct sockaddr *dest_addr, socklen_t addrlen)
{
	struct s_sendto s = {sockfd, buf, len, flags, dest_addr, addrlen};

	ssize_t ret = (ssize_t)_user_syscall(SOCKETCALL, 2, __SENDTO, &s);
	/*
	 * On success, this call returns the number of bytes sent.
	 * On error, -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
