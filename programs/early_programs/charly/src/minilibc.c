
#include "minilibc.h"

extern ssize_t user_write(int fd, const void *buf, size_t count);
extern void user_exit(int status);

ssize_t write(int fd, const void *buf, size_t count)
{
	return user_write(fd, buf, count);
}

void exit(int status)
{
	user_exit(status);
}
