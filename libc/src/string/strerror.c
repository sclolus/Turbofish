#include <ltrace.h>
#include <string.h>
#include <errno.h>

const char *err_str = "Invalid Errnum";

/*
 * strerror - return string describing error number
 */
char *strerror(int errnum)
{
	TRACE
	if (errnum < 0 || errnum >= sys_nerr) {
		errno = EINVAL;
		return (char *)err_str;
	}
	return (char *)sys_errlist[errnum];
}
