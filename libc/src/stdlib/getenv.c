#include <errno.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <i386.h>
#include <unistd.h>

extern char **environ;

char *getenv (const char *name)
{
	if (environ == NULL) {
		return NULL;
	}
	for (size_t i = 0; environ[i] != NULL; i++) {
		if (strcmp(environ[i], name) == '=') {
			return environ[i] + strlen(name) + 1;
		}
	}
	return NULL;
}
