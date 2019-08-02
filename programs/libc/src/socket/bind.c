
#include <user_syscall.h>
#include <sys/socket.h>
#include <errno.h>

extern int errno;

struct s_bind {
	int sockfd;
	const struct sockaddr *addr;
	socklen_t addrlen;
};

/*
 * Bind a name to a socket
 */
int bind(int sockfd, const struct sockaddr *addr, socklen_t addrlen)
{
	struct s_bind s = {sockfd, addr, addrlen};

	int ret = _user_syscall(SOCKETCALL, 2, __BIND, &s);
	/*
	 * On success, zero is returned.  On error, -1 is returned, and errno is set appropriately
	 */
	set_errno_and_return(ret);
}
