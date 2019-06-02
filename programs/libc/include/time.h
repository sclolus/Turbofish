#ifndef __TIME_H__
# define __TIME_H__

#include "i386.h"

typedef u32 time_t;

struct timespec {
	time_t tv_sec;        /* seconds */
	long   tv_nsec;       /* nanoseconds */
};

int nanosleep(const struct timespec *req, struct timespec *rem);

#endif
