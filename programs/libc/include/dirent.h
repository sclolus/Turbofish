#ifndef __DIRENT_H__
# define __DIRENT_H__

//The internal format of directories is unspecified.

//The <dirent.h> header shall define the following type:


typedef struct _DIR {
	int bonjour_dir;
} DIR;

//    A type representing a directory stream. The DIR type may be an incomplete type.

//It shall also define the structure dirent which shall include the following members:

#include <sys/types.h>

struct dirent {
//[XSI][Option Start]
	ino_t  d_ino   ;//    File serial number. 
//[Option End]
	char   d_name[];//    Filename string of entry. 
};

//[XSI] [Option Start] The <dirent.h> header shall define the ino_t type as described in <sys/types.h>. [Option End]

//The array d_name is of unspecified size, but shall contain a filename of at most {NAME_MAX} bytes followed by a terminating null byte.

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

int alphasort(const struct dirent **, const struct dirent **);
int closedir(DIR *);
//TODO: check that for coreutils
/* int dirfd(DIR *); */
DIR *fdopendir(int);
DIR *opendir(const char *);
struct dirent *readdir(DIR *);
int readdir_r(DIR *restrict, struct dirent *restrict, struct dirent **restrict);
void rewinddir(DIR *);
int scandir(const char *, struct dirent ***,
                   int (*)(const struct dirent *),
                   int (*)(const struct dirent **,
                   const struct dirent **));
//[XSI][Option Start]
void seekdir(DIR *, long);
long telldir(DIR *);
//[Option End]

#endif
