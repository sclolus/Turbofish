#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <ltrace.h>

# warning MISSING TESTS for fread

/*
 * binary stream input/output
 */
size_t fread(void *restrict ptr, size_t size, size_t nitems, FILE *restrict stream)
{
	TRACE
	uint8_t	*buf = (uint8_t *)ptr;

	for (size_t count = 0; count < nitems; count++) {
		for (size_t bytes = 0; bytes < size; bytes++) {
			int read;
			if (EOF == (read = fgetc(stream))) {
				return count;
			}
			buf[bytes] = (uint8_t)read;
		}
		buf += size;
	}
	/*
	 * On success, fread() return the number of items read or
	 * written. This number equals the number of bytes transferred only when
	 * size  is 1. If an error occurs, or the end of the file is reached, the
	 * return value is a short item count (or zero)
	 */
	return nitems;
}
