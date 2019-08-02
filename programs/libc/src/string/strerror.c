#include <string.h>
#include <errno.h>

char *strerror(int errnum) {

	return (char *)sys_errlist[errnum];
}
