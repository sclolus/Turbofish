
#include <ltrace.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>

/*
 * fclose - close a stream
 */
int fclose(FILE *stream)
{
	TRACE
	int fd = stream->fd;

	/*
	 * For buffering mode, send FFLUSH command to kernel
	 */

	free(stream);
	int result = close(fd);
	/*
	 * Upon successful completion, 0 is returned. Otherwise, EOF is returned and errno is set to
	 * indicate the error.  In either case, any further access (including another call to fclose()) to the stream
	 * results in undefined behavior
	 */
	if (result < 0) {
		return EOF;
	} else {
		return 0;
	}
}
