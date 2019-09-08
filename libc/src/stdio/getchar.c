#include <ltrace.h>
#include <stdio.h>

/* The getchar() function shall be equivalent to getc(stdin). */

int      getchar(void)
{
	TRACE
	return getc(stdin);
}
