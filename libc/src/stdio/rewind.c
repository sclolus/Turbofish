#include <stdio.h>
// The rewind() function sets the file position indicator for the stream pointed to by stream to the beginning of the file.  It is equivalent to:
//
// (void) fseek(stream, 0L, SEEK_SET)
void rewind(FILE *stream) {
	lseek(stream->fd, 0);
}
