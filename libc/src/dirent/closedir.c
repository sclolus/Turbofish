#include <dirent.h>
#include <errno.h>

// The closedir() function shall close the directory stream referred
// to by the argument dirp. Upon return, the value of dirp may no
// longer point to an accessible object of the type DIR. If a file
// descriptor is used to implement type DIR, that file descriptor
// shall be closed.

#warning NOT IMPLEMENTED
#include <custom.h>

int closedir(DIR *dirp)
{
	DUMMY
	(void)dirp;
	errno = ENOSYS;
	return -1;
}
