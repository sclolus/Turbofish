#include <dirent.h>
#include <errno.h>
#include <stdlib.h>

// The opendir() function shall open a directory stream corresponding
// to the directory named by the dirname argument. The directory
// stream is positioned at the first entry. If the type DIR is
// implemented using a file descriptor, applications shall only be
// able to open up to a total of {OPEN_MAX} files and directories.

#warning NOT IMPLEMENTED
#include <custom.h>

DIR *opendir(const char *dirname)
{
	DUMMY
	(void)dirname;
	errno = ENOSYS;
	return NULL;
}
