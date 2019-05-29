#include "unistd.h"

extern int user_fork();
extern int errno;

int fork()
{
	int ret = user_fork();
	if (ret < 0) {
		errno = -ret;
	}
	return ret;
}
