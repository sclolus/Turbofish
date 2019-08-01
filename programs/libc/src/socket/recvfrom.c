
#include <user_syscall.h>
#include <sys/socket.h>
#include <errno.h>

extern int errno;

struct s_recvfrom {
	int sockfd;
	void *buf;
	size_t len;
	int flags;
	struct sockaddr *dest_addr;
	socklen_t *addrlen;
};

/*
 * Receive a message from a socket
 *
 * recvfrom() places the received message into the buffer buf. The caller must
 * specify the size of the buffer in len. If src_addr is not NULL, and the underlying
 * protocol provides the source address of the message, that source address is
 * placed in the buffer pointed to by src_addr. In  this  case,  addrlen is a
 * value-result argument.
 *
 * Before the call, it should be initialized to the size of the buffer
 * associated with src_addr. Upon return, addrlen is updated to contain
 * the actual size of the source address. The returned address is truncated
 * if the buffer provided is too small; in this case, addrlen will return
 * a value greater than was supplied to the caller.
 *
 * If the caller is not interested in the source address, src_addr and addrlen should
 * be specified as NUL
 */
ssize_t recvfrom(int sockfd, void *buf, size_t len, int flags, struct sockaddr *src_addr, socklen_t *addrlen)
{
	struct s_recvfrom s = {sockfd, buf, len, flags, src_addr, addrlen};

	ssize_t ret = (ssize_t)_user_syscall(SOCKETCALL, 2, __RECVFROM, &s);
	/*
	 * This call returns the number of bytes received, or -1 if an error occurred.
	 * In the event of an error, errno is set to indicate the error.
	 * When a stream socket peer has performed an orderly shutdown,
	 * the return value will be 0 (the traditional "end-of-file" return).
	 */
	set_errno_and_return(ret);
}
