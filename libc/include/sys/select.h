#ifndef __SELECT_H__
# define __SELECT_H__

//Inclusion of the <sys/select.h> header may make visible all symbols from the headers <signal.h> and <time.h>.

#include <signal.h>
#include <time.h>
#include <sys/types.h>

//The <sys/select.h> header shall define the timeval structure, which shall include at least the following members:
struct timeval {
	time_t         tv_sec ;//    Seconds.
	suseconds_t    tv_usec;//    Microseconds.
};

//
//The <sys/select.h> header shall define the time_t and suseconds_t types as described in <sys/types.h>.
//
//The <sys/select.h> header shall define the sigset_t type as described in <signal.h>.
//
//The <sys/select.h> header shall define the timespec structure as described in <time.h>.
//
//The <sys/select.h> header shall define the fd_set type as a structure.

typedef struct _fd_set {
} fd_set;
//
//The <sys/select.h> header shall define the following symbolic constant, which shall have a value suitable for use in #if preprocessing directives:
//
#define FD_SETSIZE 0
//    Maximum number of file descriptors in an fd_set structure.
//
//The following shall be declared as functions, defined as macros, or both. If functions are declared, function prototypes shall be provided.
//
void FD_CLR(int, fd_set *);
int  FD_ISSET(int, fd_set *);
void FD_SET(int, fd_set *);
void FD_ZERO(fd_set *);

//If implemented as macros, these may evaluate their arguments more than once, so applications should ensure that the arguments they supply are never expressions with side-effects.

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

int pselect(int, fd_set *restrict, fd_set *restrict, fd_set *restrict,
		 const struct timespec *restrict, const sigset_t *restrict);
int select(int nfds,
	   fd_set *restrict readfds,
	   fd_set *restrict writefds,
	   fd_set *restrict exceptfds,
	   struct timeval *restrict timeout);

#endif
