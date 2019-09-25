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
	char	newfilename[256 * 2];

	// drop umask
	umask(0);

	snprintf(dir_filename, sizeof(dir_filename), "dir_link_is_denied_for_unwritable_dir_%u", pid);
	snprintf(filename, sizeof(filename), "%s/file_%u", dir_filename, pid);
	snprintf(newfilename, sizeof(newfilename), "%s/newfilename_%u", dir_filename, pid);

	// First creat directory without rights to write to it.
	assert(0 == mkdir(dir_filename, 0777));
	assert(-1 != open(filename, O_CREAT | O_EXCL, 0777));

	assert(0 == chmod(dir_filename, 0555));

	// We want to test the normal behavior
	assert(0 == setegid(1000));
	assert(0 == seteuid(1000));

	int ret = link(filename, newfilename);
	assert(ret == -1);
	assert(errno == EACCES);

	// We need the root rights back.
	assert(0 == setuid(0));
	assert(0 == setgid(0));

	assert(0 == chmod(dir_filename, 0777));
	assert(unlink(filename) == 0);
	assert(rmdir(dir_filename) == 0);
	return EXIT_SUCCESS;
}
