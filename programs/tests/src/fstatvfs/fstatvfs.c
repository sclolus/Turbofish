#include <sys/statvfs.h>
#include <stdio.h>
#include <fcntl.h>
#include <assert.h>
#include <unistd.h>
#include <stdlib.h>
#include <limits.h>
#include <errno.h>

int main(void)
{
	struct statvfs	buf;

	int fd = open("/bin/", O_DIRECTORY | O_RDONLY);
	assert(fd != -1);

	assert(0 == fstatvfs(fd, &buf));
	assert(buf.f_bsize != 0);
	assert(buf.f_frsize != 0);
	assert(buf.f_blocks != 0);
	assert(buf.f_bfree != 0); // That probably should be true.
	assert(buf.f_bavail != 0); // That probably should be true.
	assert(buf.f_files != 0);
	assert(buf.f_ffree != 0);
	assert(buf.f_favail != 0);
	assert(buf.f_namemax == NAME_MAX - 1);
	assert(close(fd) == 0);
	return EXIT_SUCCESS;
}
