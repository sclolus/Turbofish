#include <stdio.h>
#include <custom.h>

#warning DUMMY IMPLEMENTATION

FILE *fopen(const char *restrict pathname, const char *restrict mode)
{
	DUMMY
	(void)pathname;
	(void)mode;
	return stdout;
}
