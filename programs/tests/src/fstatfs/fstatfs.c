#include <sys/statfs.h>
#include <fcntl.h>
#include <stdio.h>
#include <assert.h>
#include <unistd.h>
#include <stdlib.h>
#include <limits.h>
#include <errno.h>

int main(void)
{
	struct statfs	buf;

	int fd = open("/bin/", O_DIRECTORY | O_RDONLY);
	assert(fd != -1);

	assert(0 == fstatfs(fd, &buf));
	assert(buf.f_bsize != 0);
	assert(buf.f_frsize != 0);
	assert(buf.f_blocks != 0);
	assert(buf.f_bfree != 0); // That probably should be true.
	assert(buf.f_bavail != 0); // That probably should be true.
	assert(buf.f_files != 0);
	assert(buf.f_ffree != 0);
	assert(buf.f_type == EXT2_SUPER_MAGIC); // Assumes that the root filesytem is ext2.
	assert(buf.f_namelen == NAME_MAX - 1);
	assert(close(fd) == 0);
	return EXIT_SUCCESS;
}
