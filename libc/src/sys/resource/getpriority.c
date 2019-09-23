#include <sys/resource.h>
#include <errno.h>

#warning DUMMY IMPLEMENTATION

int getpriority(int which, id_t who)
{
	(void)which;
	(void)who;
	errno = ENOSYS;
	return -1;
}
