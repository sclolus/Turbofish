#include <errno.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <i386.h>
#include <unistd.h>

extern char **environ;

#warning DUMMY IMPLEMENTATION

char *getenv (const char *name)
{
	(void)name;
	return NULL;
}
