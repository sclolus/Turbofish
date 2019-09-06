#include <stdio.h>

/* Versions of the functions getc(), getchar(), putc(), and putchar()
 respectively named getc_unlocked(), getchar_unlocked(), putc_unlocked(), and putchar_unlocked()
 shall be provided which are functionally equivalent to the original versions,
with the exception that they are not required to be implemented in a fully thread-safe manner. */
int      getchar_unlocked(void)
{
	return getchar();
}
