#include <stdio.h>
#include <custom.h>

int ungetc(int c, FILE *stream)
{
	DUMMY
	(void)c;
	(void)stream;
	return EOF;
}
