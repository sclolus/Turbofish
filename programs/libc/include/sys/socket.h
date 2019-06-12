#ifndef __SOCKET_H__
# define __SOCKET_H__

#include <i386.h>

/*
 * LIBC internal values for user_socketcall
 */
#define __SOCKET      0
#define __BIND        1
#define __CONNECT     2
#define __LISTEN      3
#define __ACCEPT      4
#define __SEND        5
#define __RECV        6
#define __SENDTO      7
#define __RECVFROM    9
#define __SHUTDOWN    9

/*
 * sun_family
 */
#define AF_UNIX 0

/*
 * type
 */
#define SOCK_STREAM 0 // Connection-oriented
#define SOCK_DGRAM 1  // Connectionless

#define UNIX_PATHNAME_MAXSIZE 300

struct sockaddr;      // Opaque pointer to avoid compilation errors or warnings

/*
 * Unix socket sockaddr interface (AF_UNIX)
 */
struct sockaddr_un {
	u16 sun_family;
	u8 unix_pathname[UNIX_PATHNAME_MAXSIZE];
};

typedef size_t socklen_t;

int socket(int domain, int type, int protocol);
int bind(int sockfd, const struct sockaddr *addr, socklen_t addrlen);
int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen);
int listen(int sockfd, int backlog);
int accept(int sockfd, struct sockaddr *addr, socklen_t *addrlen);
ssize_t send(int sockfd, const void *buf, size_t len, int flags);
ssize_t recv(int sockfd, void *buf, size_t len, int flags);
ssize_t sendto(int sockfd, const void *buf, size_t len, int flags, const struct sockaddr *dest_addr, socklen_t addrlen);
ssize_t recvfrom(int sockfd, void *buf, size_t len, int flags, struct sockaddr *src_addr, socklen_t *addrlen);
int shutdown(int sockfd, int how);

#endif
