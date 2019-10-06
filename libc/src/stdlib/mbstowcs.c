#include <stdlib.h>
#include <custom.h>
#include <errno.h>

size_t mbstowcs(wchar_t *dest, const char *src, size_t n)
{
	DUMMY
		(void)dest;
	(void)src;
	(void)n;
	errno = ENOSYS;
	return (size_t)-1;
}
