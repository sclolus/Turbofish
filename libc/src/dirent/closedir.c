#include <dirent.h>
#include <errno.h>
#include <stdlib.h>
#include <sys/mman.h>

// The closedir() function shall close the directory stream referred
// to by the argument dirp. Upon return, the value of dirp may no
// longer point to an accessible object of the type DIR. If a file
// descriptor is used to implement type DIR, that file descriptor
// shall be closed.

int closedir(DIR *dirp)
{
	if (munmap(dirp->array, dirp->length * sizeof(struct dirent)) < 0) {
		return -1;
	}
	free(dirp);
	return 0;
}
