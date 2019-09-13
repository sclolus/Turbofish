#include <unistd.h>


/* DESCRIPTION */
/* The sync() function shall cause all information in memory that updates file systems to be scheduled for writing out to all file systems. */

/* The writing, although scheduled, is not necessarily complete upon return from sync(). */

/* RETURN VALUE */
/* The sync() function shall not return a value. */

#warning DUMMY IMPLEMENTATION

#include <custom.h>

void         sync(void)
{
	DUMMY_KERNEL
	DUMMY
}
