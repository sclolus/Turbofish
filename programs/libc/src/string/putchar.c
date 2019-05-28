
#include "string.h"
#include "unistd.h"

extern int write(int fd, const char *buf, size_t count);

void	putchar(char c)
{
	write(1, &c, 1);
}
