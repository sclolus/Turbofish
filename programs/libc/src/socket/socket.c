
#include <user_syscall.h>
#include <sys/socket.h>

extern int errno;

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
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return ret;
	}
}
