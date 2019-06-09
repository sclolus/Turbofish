
#include "time.h"

extern int user_nanosleep(const struct timespec *req, struct timespec *rem);
extern int errno;

int nanosleep(const struct timespec *req, struct timespec *rem) {
	int ret = user_nanosleep(req, rem);
	if (ret < 0) {
		errno = -ret;
		return -1;
	}
	return 0;
}
