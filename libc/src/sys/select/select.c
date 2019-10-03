#include <user_syscall.h>
#include <errno.h>
#include <sys/select.h>
#include <time.h>
#include <ltrace.h>
#include <custom.h>

#warning "dummy implementation: only timeout parameter is supported"
int select(int nfds,
	   fd_set *restrict readfds,
	   fd_set *restrict writefds,
	   fd_set *restrict exceptfds,
	   struct timeval *restrict timeout) {
	TRACE
	DUMMY
	if (nfds != 0
	    || readfds != NULL
	    || writefds != NULL
	    || exceptfds != NULL
	    || timeout == NULL) {
		errno = ENOSYS;
		return -1;
	}

	struct timespec t;

	t.tv_sec = timeout->tv_sec;
	t.tv_nsec = (long)timeout->tv_usec;
	// nanosleep might not sleep all the duration of timeout.
	// We could detect that and retry, but that's not necessary
	// for now.
	return nanosleep(&t, NULL);
}
