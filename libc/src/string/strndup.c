#include <ltrace.h>
#include <string.h>
#include <stdlib.h>


/*
 *
 * The strndup() function shall be equivalent to the strdup() function,
 * duplicating the provided s in a new block of memory allocated as if by using malloc(),
 * with the exception being that strndup() copies at most size plus one bytes into the newly allocated memory,
 * terminating the new string with a NUL character. If the length of s is larger than size, only size bytes shall be duplicated.
 * If size is larger than the length of s, all bytes in s shall be copied into the new memory buffer,
 * including the terminating NUL character. The newly created string shall always be properly terminated.
 */
char    *strndup(const char *s, size_t size)
{
	TRACE
	size_t s_len = strlen(s);
	size_t len = s_len > size ? size : s_len;
	char *new = (char *)malloc(len + 1);
	if (new == NULL)
		return NULL;
	new[len] = '\0';
	return (char *)memcpy(new, s, len);
}
