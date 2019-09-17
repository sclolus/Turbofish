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

	// We want to test the normal behavior
	assert(0 == seteuid(1000));
	assert(0 == setegid(1000));

	snprintf(dir_filename, sizeof(dir_filename), "dir_symlink_is_denied_for_unwritable_dir_%u", pid);
	snprintf(filename, sizeof(filename), "%s/symlink_%u", dir_filename, pid);

	// First creat directory without rights to write to it.
	assert(0 == mkdir(dir_filename, 0555));
	int ret = symlink(dir_filename, filename);

	assert(ret == -1);
	assert(errno == EACCES);
	assert(rmdir(dir_filename) == 0);
	return EXIT_SUCCESS;
}
