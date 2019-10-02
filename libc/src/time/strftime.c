#include <ltrace.h>
#include <time.h>
#include <errno.h>
#include <custom.h>

size_t strftime(char *restrict s, size_t maxsize,
       const char *restrict format, const struct tm *restrict timeptr)
{
	TRACE
	DUMMY
	errno = ENOSYS;
	return -1;
}
