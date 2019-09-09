#include <ltrace.h>
#include <sys/stat.h>

#warning MKFIFO FUNCTION MUST BE DEFINED
#include <custom.h>

int mkfifo(const char *path, mode_t mod)
{
	TRACE
	DUMMY
	(void)path;
	(void)mod;
	return 0;
}
