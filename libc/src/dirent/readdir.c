#include <dirent.h>
#include <errno.h>
#include <stdlib.h>

// The readdir() function shall return a pointer to a structure
// representing the directory entry at the current position in the
// directory stream specified by the argument dirp, and position the
// directory stream at the next entry. It shall return a null pointer
// upon reaching the end of the directory stream. The structure dirent
// defined in the <dirent.h> header describes a directory entry. The
// value of the structure's d_ino member shall be set to the file
// serial number of the file named by the d_name member. If the d_name
// member names a symbolic link, the value of the d_ino member shall
// be set to the file serial number of the symbolic link itself.

struct dirent *readdir(DIR *dirp)
{
	/*
	 * On success, readdir() returns a pointer to a dirent structure. (This structure may be statically allocated;
	 * do not attempt to free(3) it.)
	 *
	 * If the end of the directory stream is reached, NULL is returned and errno is not changed.
	 * If an error occurs, NULL is returned and errno is set appropriately. To distinguish end of stream and from
	 * an error, set errno to zero before calling readdir() and then check the value of errno if NULL is returned.
	 */
	if (dirp->current_offset < dirp->length) {
		struct dirent *dirent = &(dirp->array[dirp->current_offset]);
		dirp->current_offset += 1;
		return dirent;
	} else {
		return NULL;
	}
}
