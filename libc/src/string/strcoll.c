/*
 * strcoll - compare two strings using the current locale
 */

#include <ltrace.h>
#include <string.h>

int strcoll(const char *s1, const char *s2)
{
	TRACE
	/*
	 * In the POSIX or C locales strcoll() is equivalent to strcmp(3).
	 */
	return strcmp(s1, s2);
}
