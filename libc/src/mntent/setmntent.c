#include <mntent.h>
#include <stdio.h>

/* The  setmntent() function opens the filesystem description file filename and returns */
/* a file pointer which can be used by getmntent().  The argument type is the  type  of */
/* access required and can take the same values as the mode argument of fopen(3). */
FILE *setmntent(const char *filename, const char *type)
{
	return fopen(filename, type);
}
