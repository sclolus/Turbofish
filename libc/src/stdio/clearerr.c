#include <stdio.h>
#include <stdbool.h>

void clearerr(FILE *stream)
{
	stream->error = false;
	stream->eof = false;
}
