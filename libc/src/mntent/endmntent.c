#include <mntent.h>
#include <stdio.h>

/* The  endmntent()  function closes the stream associated with the filesystem descrip‐ */
/* tion file. */

int endmntent(FILE *streamp)
{
	fclose(streamp); //ignore return.
	return 1;
}
