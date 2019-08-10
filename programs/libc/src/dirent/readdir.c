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

#warning NOT IMPLEMENTED

struct dirent *readdir(DIR *dirp)
{
	errno = ENOSYS;
	return NULL;
}
