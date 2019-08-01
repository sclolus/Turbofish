
#include <user_syscall.h>
#include <sys/socket.h>
#include <errno.h>

extern int errno;

struct s_accept {
	int sockfd;
	struct sockaddr *addr;
	socklen_t *addrlen;
};

/*
 * Accept a connection on a socket
 *
 * The argument addr is a pointer to a sockaddr structure. This structure is filled
 * in with the address of the peer socket, as known to the communications layer.
 * The exact format of the address returned addr is determined by the socket's address
 * family (see socket(2) and the respective protocol man pages). When addr is NULL,
 * nothing is filled in; in this case, addrlen is not used, and should also be NULL.
 */
int accept(int sockfd, struct sockaddr *addr, socklen_t *addrlen)
{
	struct s_accept s = {sockfd, addr, addrlen};

	int ret = _user_syscall(SOCKETCALL, 2, __ACCEPT, &s);
	/*
	 * On success, these system calls return a nonnegative integer that is a file
	 * descriptor for the accepted socket.  On error, -1 is returned,
	 * and errno is set appropriately
	 */
	set_errno_and_return(ret);
}
