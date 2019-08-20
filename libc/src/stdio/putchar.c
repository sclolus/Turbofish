#include <unistd.h>

/*
 * putchar(c) is equivalent to putc(c, stdout).
 */
int putchar(int c)
{
	return (int)write(1, &c, 1);
}
