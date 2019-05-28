
#include "unistd.h"

extern int user_write(int fd, const char *s, size_t len);

int write(int fd, const char *s, size_t len)
{
	return user_write(fd, s, len);
}
