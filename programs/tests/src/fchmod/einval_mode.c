#include <sys/stat.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>
#include <assert.h>
#include <errno.h>
#include <stdlib.h>

int main(void)
{
	pid_t	pid = getpid();
	char	filename[256];

	snprintf(filename, sizeof(filename), "test_invalid_chmod_%u", pid);

	int fd = open(filename, O_CREAT | O_EXCL, 0);
	assert(-1 != fd);
	assert(-1 == fchmod(fd, 0111777));
	assert(errno == EINVAL);
	assert(0 == close(fd));
	assert(0 == unlink(filename));
	return EXIT_SUCCESS;
}
