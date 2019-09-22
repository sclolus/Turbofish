#include <stdint.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <errno.h>


int main(void)
{
	pid_t	pid = getpid();
	char	dir_filename[256];
	char	filename[256 * 2];

	// drop umask
	umask(0);
	assert(0 == seteuid(1000));
	assert(0 == setegid(1000));

	snprintf(dir_filename, sizeof(dir_filename), "dir_pathname_resolution_eaccess_basic_%u", pid);
	snprintf(filename, sizeof(filename), "%s/test_pathname_resolution_eaccess_basic_%u", dir_filename, pid);
	// First creat directory with rights to write to it.
	assert(0 == mkdir(dir_filename, 0777));

	int fd = open(filename, O_CREAT | O_EXCL, 0666);
	assert(fd != -1);
	assert(close(fd) == 0);
	assert(0 == chmod(dir_filename, 0666));

	// try to reopen it
	fd = open(filename, O_RDONLY, 0666);
	assert(-1 == fd);
	assert(errno == EACCES);

	// retake search permissions
	assert(0 == chmod(dir_filename, 0777));
	assert(0 == unlink(filename));
	assert(rmdir(dir_filename) == 0);
	return EXIT_SUCCESS;
}
