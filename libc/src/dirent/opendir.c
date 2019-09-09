#include <ltrace.h>
#include <dirent.h>
#include <errno.h>
#include <stdlib.h>
#include <user_syscall.h>

// The opendir() function shall open a directory stream corresponding
// to the directory named by the dirname argument. The directory
// stream is positioned at the first entry. If the type DIR is
// implemented using a file descriptor, applications shall only be
// able to open up to a total of {OPEN_MAX} files and directories.

DIR *opendir(const char *dirname)
{
	TRACE
	DIR *dir = (DIR *)malloc(sizeof(DIR));
	if (dir == NULL) {
		return NULL;
	}
	int ret = _user_syscall(OPENDIR, 2, dirname, dir);
	/*
	 * The opendir() and fdopendir() functions return a pointer to the directory stream.
	 * On error, NULL is returned, and errno is set appropriately.
	 */
	if (ret < 0) {
		errno = -ret;
		free(dir);
		return NULL;
	} else {
		return dir;
	}
}
