#include <stdio.h>
#include <string.h>
#include <stdint.h>

# warning MISSING TESTS for fread

size_t fread(void *restrict ptr, size_t size, size_t nitems,
       FILE *restrict stream)
{
	size_t	count = 0;
	uint8_t	*buf;

	while (count < nitems) {
		size_t bytes = 0;
		int    read;

		while (bytes < size) {
			if (EOF == (read = fgetc(stream))) {
				return count;
			}

			buf[bytes] = (uint8_t)read;
			bytes++;
		}
		buf += size;
		count++;
	}
	return count;
}
