#include <ltrace.h>
#include <user_syscall.h>
#include <sys/socket.h>
#include <errno.h>

struct s_listen {
	int sockfd;
	int backlog;
};

/*
 * Listen for connections on a socket
 */
int listen(int sockfd, int backlog)
{
	TRACE
	struct s_listen s = {sockfd, backlog};

	int ret = _user_syscall(SOCKETCALL, 2, __LISTEN, &s);
	/*
	 * On success, zero is returned.  On error, -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
