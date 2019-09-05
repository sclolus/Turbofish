#ifndef __SYS_TIME_H__
# define __SYS_TIME_H__


#include <sys/types.h>
//The <sys/time.h> header shall define the time_t and suseconds_t types as described in <sys/types.h>.

//The <sys/time.h> header shall define the timeval structure, which shall include at least the following members:
struct timeval {
	time_t         tv_sec ;//    Seconds.
	suseconds_t    tv_usec;//    Microseconds.
};

#include <sys/select.h>
//The <sys/time.h> header shall define the fd_set type as described in <sys/select.h>.

//[OB] [Option Start] The <sys/time.h> header shall define the itimerval structure, which shall include at least the following members:

struct itimerval {
	struct timeval it_interval;// Timer interval.
	struct timeval it_value   ;// Current value.
};

//[Option End]

//[OB] [Option Start] The <sys/time.h> header shall define the following symbolic constants for the which argument of getitimer() and setitimer():
//
//ITIMER_REAL
//    Decrements in real time.
//ITIMER_VIRTUAL
//    Decrements in process virtual time.
//ITIMER_PROF
//    Decrements both in process virtual time and when the system is running on behalf of the process.
//
//[Option End]

//The <sys/time.h> header shall define the following as described in <sys/select.h>: FD_CLR() FD_ISSET() FD_SET() FD_ZERO() FD_SETSIZE

//he following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

//[OB][Option Start]
int getitimer(int, struct itimerval *);
int gettimeofday(struct timeval *restrict, void *restrict);
int setitimer(int, const struct itimerval *restrict,
		struct itimerval *restrict);
//[Option End]
//int   select(int, fd_set *restrict, fd_set *restrict, fd_set *restrict,
//		struct timeval *restrict);
int utimes(const char *, const struct timeval [2]);

//TODO: check that not posiz
struct timezone {
	int tz_minuteswest;     /* minutes west of Greenwich */
	int tz_dsttime;         /* type of DST correction */
};

#endif
