#include <stdio.h>

/* The getchar() function shall be equivalent to getc(stdin). */

int      getchar(void)
{
	return getc(stdin);
}
