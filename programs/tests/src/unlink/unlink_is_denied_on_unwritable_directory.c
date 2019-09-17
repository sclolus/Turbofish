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

	snprintf(dir_filename, sizeof(dir_filename), "dir_open_o_creat_is_denied_for_unwritable_dir_%u", pid);
	snprintf(filename, sizeof(filename), "%s/no_unlink_%u", dir_filename, pid);

	// First creat directory with rights to write first.
	assert(0 == mkdir(dir_filename, 0777));
	int fd = open(filename, O_CREAT | O_EXCL, 0666);
	assert(fd != -1);
	// Now, removes write permissions on the parent directory
	assert(0 == chmod(dir_filename, 0555));

	// try to unlink the file.
	assert(-1 == unlink(filename));
	assert(errno == EACCES);

	// undo all the stuff.
	assert(0 == chmod(dir_filename, 0777));
	assert(0 == unlink(filename));
	assert(rmdir(dir_filename) == 0);
	return EXIT_SUCCESS;
}
