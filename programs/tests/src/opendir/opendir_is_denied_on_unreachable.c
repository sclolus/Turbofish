#include <stdint.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <errno.h>
#include <dirent.h>
#include <tools/tools.h>


int main(void)
{
	pid_t	pid = getpid();
	char	dir_filename[256];

	// drop umask
	umask(0);

	snprintf(dir_filename, sizeof(dir_filename), "dir_opendir_is_denied_for_unwritable_dir_%u", pid);
	// First creat directory without rights to read to it.
	if (0 != mkdir(dir_filename, 0333)) {
		err_errno("Failed to mkdir %s", dir_filename);
	}
	// We want to test the normal behavior
	assert(0 == seteuid(1000));
	assert(0 == setegid(1000));


	assert(NULL == opendir(dir_filename));
	assert(errno == EACCES);
	assert(rmdir(dir_filename) == 0);
	return EXIT_SUCCESS;
}
