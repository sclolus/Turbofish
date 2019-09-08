#include <ltrace.h>
#include <unistd.h>

/*
 * putchar(c) is equivalent to putc(c, stdout).
 */
int putchar(int c)
{
	TRACE
	return (int)write(1, &c, 1);
}
