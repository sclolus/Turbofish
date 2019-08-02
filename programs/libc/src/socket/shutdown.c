
#include <user_syscall.h>
#include <sys/socket.h>

extern int errno;

struct s_shutdown {
	int sockfd;
	int how;
};

/*
 * Shut down part of a full-duplex connection
 */
int shutdown(int sockfd, int how)
{
	struct s_shutdown s = {sockfd, how};

	int ret = _user_syscall(SOCKETCALL, 2, __SHUTDOWN, &s);
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
