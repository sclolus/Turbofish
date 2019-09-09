#include <ltrace.h>
#include <stdlib.h>

/*
 * abs - compute the absolute value of an integer
 */
int abs(int j)
{
	TRACE
	return j < 0 ? -j : j;
}
