#include <stdio.h>
#include <unistd.h>

int fseek(FILE *stream, long offset, int whence)
{
	return lseek(stream->fd, offset, whence);
}
