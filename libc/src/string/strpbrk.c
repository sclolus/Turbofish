#include <ltrace.h>
#include <string.h>

/*
 * Find the first occurrence in S of any character in ACCEPT.
 */
char *strpbrk(const char *s, const char *accept)
{
	TRACE
	s += strcspn(s, accept);
	return *s ? (char *)s : NULL;
}
