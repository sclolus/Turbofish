#include <string.h>
#include <stdlib.h>

/*
 * strdup - duplicate a string
 */
char    *strdup(const char *s)
{
	size_t len = strlen(s) + 1;
	char *new = (char *)malloc(len);
	if (new == NULL)
		return NULL;
	new[len - 1] = '\0';
	return (char *)memcpy(new, s, len - 1);
}
