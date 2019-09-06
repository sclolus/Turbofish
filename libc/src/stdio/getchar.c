#include <stdio.h>
#include <errno.h>

/* The getchar() function shall be equivalent to getc(stdin). */

int      getchar(void)
{
	return getc(stdin);
}

int      getchar_unlocked(void)
{
	return getchar();
}
