#include <sys/stat.h>
#include <unistd.h>
#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <fcntl.h>

int main(void)
{
	pid_t pid = getpid();
	char  filename[256];

	snprintf(filename, sizeof(filename), "test_fchmod_normal_usage_%u", pid);
	int fd = open(filename, O_CREAT | O_EXCL, 0);
	assert(fd != 0);

	struct stat buf;
	mode_t	mode = 01234;
	assert(0 == fchmod(fd, 01234));
	assert(0 == fstat(fd, &buf));
	mode_t got = buf.st_mode & 07777;
	assert(got == mode);

	mode = 0777;
	assert(0 == fchmod(fd, mode));
	assert(0 == fstat(fd, &buf));
	got = buf.st_mode & 07777;
	assert(got == mode);

	mode = 07777;
	assert(0 == fchmod(fd, mode));
	assert(0 == fstat(fd, &buf));
	got = buf.st_mode & 07777;
	assert(got == mode);

	mode = 0755;
	assert(0 == fchmod(fd, mode));
	assert(0 == fstat(fd, &buf));
	got = buf.st_mode & 07777;
	assert(got == mode);

	mode = 04041;
	assert(0 == fchmod(fd, mode));
	assert(0 == fstat(fd, &buf));
	got = buf.st_mode & 07777;
	assert(got == mode);

	assert(0 == close(fd));
	assert(0 == unlink(filename));
	return EXIT_SUCCESS;
}
