#include <sys/resource.h>
#include <errno.h>
#include <custom.h>
#include <ltrace.h>

#warning DUMMY IMPLEMENTATION

int setpriority(int which, id_t who, int value)
{
	TRACE
	DUMMY
	(void)which;
	(void)who;
	(void)value;

	errno = ENOSYS;
	return -1;
}
