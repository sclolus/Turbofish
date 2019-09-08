#include <sys/statvfs.h>
#include <stdio.h>
#include <assert.h>
#include <unistd.h>
#include <stdlib.h>
#include <limits.h>
#include <errno.h>

int main(void)
{
	struct statvfs	buf;

	assert(-1 == statvfs("/non_existante_entry/make/sure/of/that/bullshit", &buf));
	assert(errno == ENOENT);
	assert(0 == statvfs("/bin", &buf));
	assert(buf.f_bsize != 0);
	assert(buf.f_frsize != 0);
	assert(buf.f_blocks != 0);
	assert(buf.f_bfree != 0); // That probably should be true.
	assert(buf.f_bavail != 0); // That probably should be true.
	assert(buf.f_files != 0);
	assert(buf.f_ffree != 0);
	assert(buf.f_favail != 0);
	assert(buf.f_namemax == NAME_MAX);
	return EXIT_SUCCESS;
}
