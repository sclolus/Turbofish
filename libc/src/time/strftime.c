#include <ltrace.h>
#include <time.h>
#include <errno.h>

size_t strftime(char *restrict s, size_t maxsize,
       const char *restrict format, const struct tm *restrict timeptr)
{
	TRACE
	errno = ENOSYS;
	return 0;
}
