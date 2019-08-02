#ifndef __SOCKET_H__
# define __SOCKET_H__

#include <stddef.h>
#include <sys/types.h>

/*
 * LIBC internal values for user_socketcall
 */
#define __SOCKET      1
#define __BIND        2
#define __CONNECT     3
#define __LISTEN      4
#define __ACCEPT      5
#define __SEND        9
#define __RECV        10
#define __SENDTO      11
#define __RECVFROM    12
#define __SHUTDOWN    13

/*
 * sun_family
 */
#define AF_UNIX 1

/*
 * type
 */
#define SOCK_STREAM 1 // Connection-oriented
#define SOCK_DGRAM 2  // Connectionless

struct sockaddr;      // Opaque pointer to avoid compilation errors or warnings

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
