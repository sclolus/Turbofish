#include <getopt.h>
#include <unistd.h>
#include <errno.h>
#include <ltrace.h>

# warning DUMMY IMPLEMENTATION of getopt_long

int getopt_long(int argc, char **argv,
			const char *shortopts,
		const struct option *longopts, int *longind)
{
	TRACE
	opterr = 1;
	errno = ENOSYS;
	return -1;
}
