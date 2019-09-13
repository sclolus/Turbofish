#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <assert.h>
#include <stdio.h>
#include <utime.h>
#include <time.h>
#include <stdint.h>
#include <stdlib.h>

#define NBR_TEST 100
#define INCR 3
int main(void)
{
	pid_t	pid = getpid();
	char	filename[256];

	snprintf(filename, sizeof(filename), "test_utime_basic_%u", pid);

	int fd = open(filename, O_CREAT | O_EXCL, 0);

	assert(-1 != fd);

	for (uint32_t i = 0; i < NBR_TEST; i++) {
		struct stat	    buf;
		struct utimbuf	    utimbuf;
		time_t		    current_time;

		// We should use rand() to get something more useful.
		assert((time_t)-1 != time(&current_time));

		// Since this is going to be the same value each time, multiply it by something.
		current_time *= (time_t)i;

		utimbuf.actime = current_time;
		utimbuf.modtime = current_time;


		assert(0 == utime(filename, &utimbuf));
		assert(0 == fstat(fd, &buf));

		assert(current_time == buf.st_atime);
		assert(current_time == buf.st_mtime);
	}
	assert(0 == close(fd));
	assert(0 == unlink(filename));
	return EXIT_SUCCESS;
}
