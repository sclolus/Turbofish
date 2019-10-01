#include <getopt.h>
#include <unistd.h>
#include <errno.h>
#include <ltrace.h>
#include <custom.h>

# warning DUMMY IMPLEMENTATION of getopt_long

int getopt_long(int argc, char **argv,
			const char *shortopts,
		const struct option *longopts, int *longind)
{
	TRACE
	DUMMY
	opterr = 1;
	errno = ENOSYS;
	return -1;
}
