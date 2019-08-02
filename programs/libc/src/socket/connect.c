
#include <user_syscall.h>
#include <sys/socket.h>

extern int errno;

struct s_connect {
	int sockfd;
	const struct sockaddr *addr;
	socklen_t addrlen;
};

/*
 * Initiate a connection on a socket
 */
int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen)
{
	struct s_connect s = {sockfd, addr, addrlen};

	int ret = _user_syscall(SOCKETCALL, 2, __CONNECT, &s);
	/*
	 * If the connection or binding succeeds, zero is returned.
	 * On error, -1 is returned, and errno is set appropriately.
	 */
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return 0;
	}
}
