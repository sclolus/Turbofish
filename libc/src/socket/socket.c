#include <user_syscall.h>
#include <sys/socket.h>
#include <errno.h>

struct s_socket {
	int domain;
	int type;
	int protocol;
};

/*
 * Create an endpoint for communication
 */
int socket(int domain, int type, int protocol)
{
	struct s_socket s = {domain, type, protocol};

	int ret = _user_syscall(SOCKETCALL, 2, __SOCKET, &s);
	/*
	 * On success, a file descriptor for the new socket is returned.
	 * On error, -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
