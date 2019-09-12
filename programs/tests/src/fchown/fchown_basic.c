#include <unistd.h>
#include <sys/stat.h>
#include <errno.h>
#include <stdio.h>
#include <assert.h>
#include <stdint.h>
#include <fcntl.h>
#include <stdlib.h>

#define NBR_TEST 100
#define INCR 3
int main(void)
{
	pid_t	pid = getpid();
	char	filename[256];

	snprintf(filename, sizeof(filename), "test_chown_basic_%u", pid);

	int fd = open(filename, O_CREAT | O_EXCL, 0);

	assert(-1 != fd);

	for (uint32_t i = 0; i < NBR_TEST; i++) {
		struct stat buf;
		uid_t new_owner = i * INCR;
		gid_t new_group = i * INCR;

		assert(0 == fchown(fd, new_owner, new_group));
		assert(0 == fstat(fd, &buf));

		assert(new_owner == buf.st_uid);
		assert(new_group == buf.st_gid);
	}
	assert(0 == close(fd));
	assert(0 == unlink(filename));
	return EXIT_SUCCESS;
}
