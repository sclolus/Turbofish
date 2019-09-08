#include <stdio.h>

int getc_unlocked(FILE *stream)
{
	return getc(stream);
}
