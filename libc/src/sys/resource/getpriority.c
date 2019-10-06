#include <sys/resource.h>
#include <errno.h>
#include <ltrace.h>
#include <custom.h>

int getpriority(int which, id_t who)
{
	TRACE
	DUMMY
	(void)which;
	(void)who;
	errno = ENOSYS;
	return -1;
}
