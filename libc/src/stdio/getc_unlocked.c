#include <ltrace.h>
#include <stdio.h>

int getc_unlocked(FILE *stream)
{
	TRACE
	return getc(stream);
}
