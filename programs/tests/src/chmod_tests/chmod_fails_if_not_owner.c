#include <stdlib.h>
#include <unistd.h>
#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <fcntl.h>
#include <sys/stat.h>


int main(void)
{
	pid_t	pid = getpid();
	char	filename[256];

	snprintf(filename, sizeof(filename), "chmod_fails_if_not_owner_%u", pid);
	int fd = open(filename, O_CREAT | O_EXCL, 0777);

	assert(fd != -1);
	assert(0 == close(fd));

	assert(0 == setegid(1000));
	assert(0 == seteuid(1000));

	assert(-1 == chmod(filename, 0000));
	assert(errno == EPERM);

	assert(0 == seteuid(0));
	assert(0 == setegid(0));
	assert(0 == unlink(filename));
	return EXIT_SUCCESS;
}
