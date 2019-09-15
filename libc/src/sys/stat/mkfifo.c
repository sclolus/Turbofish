#include <ltrace.h>
#include <sys/stat.h>

int mkfifo(const char *path, mode_t mod)
{
	return mknod(path, S_IFIFO | (mod & (S_IRWXU | S_IRWXG | S_IRWXO)), 0);
}
