#include <stdlib.h>
#include <assert.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <fcntl.h>
#include <errno.h>

int main(void)
{
	pid_t	pid = getpid();
	char	filename[256];


	// drop umask
	umask(0);


	snprintf(filename, sizeof(filename), "cannot_open_with_incorrect_file_%u", pid);

	int fd = open(filename, O_CREAT | O_EXCL, 0444);
	assert(fd != -1);

	// We want to test the normal behavior
	assert(0 == seteuid(1000));
	assert(0 == setegid(1000));

	assert(0 == close(fd));
	fd = open(filename, O_WRONLY);
	assert(fd == -1);
	assert(errno == EACCESS);
	fd = open(filename, O_RDWR);
	assert(fd == -1);
	assert(errno == EACCESS);


	/// Remove read permissions , put write permissions.
	assert(0 == chmod(filename, 0222));

	fd = open(filename, O_RDONLY);
	assert(fd == -1);
	assert(errno == EACCESS);

	fd = open(filename, O_RDWR);
	assert(errno == EACCESS);
	assert(fd == -1);

	assert(0 == seteuid(0));
	assert(0 == setegid(0));

	assert(0 == unlink(filename));
	return EXIT_SUCCESS;

}
