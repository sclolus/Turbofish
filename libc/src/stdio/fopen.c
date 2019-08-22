#include <stdio.h>
#include <custom.h>

#warning DUMMY IMPLEMENTATION

FILE *fopen(const char *pathname, const char *mode)
{
	DUMMY
	(void)pathname;
	(void)mode;
	return stdout;
}
