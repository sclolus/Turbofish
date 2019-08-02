
#include <user_syscall.h>
#include <sys/socket.h>

extern int errno;

struct s_listen {
	int sockfd;
	int backlog;
};

/*
 * Listen for connections on a socket
 */
int listen(int sockfd, int backlog)
{
	struct s_listen s = {sockfd, backlog};

	int ret = _user_syscall(SOCKETCALL, 2, __LISTEN, &s);
	/*
	 * On success, zero is returned.  On error, -1 is returned, and errno is set appropriately.
	 */
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return 0;
	}
}
