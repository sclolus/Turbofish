#include <string.h>

/*
 * Find the first occurrence in S of any character in ACCEPT.
 */
char *strpbrk(const char *s, const char *accept)
{
	s += strcspn(s, accept);
	return *s ? (char *)s : NULL;
}
