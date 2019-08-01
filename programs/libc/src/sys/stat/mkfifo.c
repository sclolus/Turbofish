
#include <sys/stat.h>

#include <custom.h>

#warning MKFIFO FUNCTION MUST BE DEFINED

int mkfifo(const char *path, mode_t mod) {
	FUNC
	(void)path;
	(void)mod;
	return 0;
}
