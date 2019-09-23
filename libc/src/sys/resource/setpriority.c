#include <sys/resource.h>
#include <errno.h>

#warning DUMMY IMPLEMENTATION

int setpriority(int which, id_t who, int value)
{
	(void)which;
	(void)who;
	(void)value;

	errno = ENOSYS;
	return -1;
}
