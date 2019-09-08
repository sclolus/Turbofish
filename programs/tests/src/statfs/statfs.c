#include <sys/statfs.h>
#include <stdio.h>
#include <assert.h>
#include <unistd.h>
#include <stdlib.h>
#include <limits.h>
#include <errno.h>

int main(void)
{
	struct statfs	buf;

	assert(-1 == statfs("/non_existante_entry/make/sure/of/that/bullshit", &buf));
	assert(errno == ENOENT);
	assert(0 == statfs("/bin", &buf));
	assert(buf.f_bsize != 0);
	assert(buf.f_frsize != 0);
	assert(buf.f_blocks != 0);
	assert(buf.f_bfree != 0); // That probably should be true.
	assert(buf.f_bavail != 0); // That probably should be true.
	assert(buf.f_files != 0);
	assert(buf.f_ffree != 0);
	assert(buf.f_type == EXT2_SUPER_MAGIC); // Assumes that the root filesytem is ext2.
	assert(buf.f_namelen == NAME_MAX - 1);
	return EXIT_SUCCESS;
}
