#include <stdio_ext.h>
#include <ltrace.h>

#warning DUMMY_IMPLEMENTATION of __fpending

size_t __fpending(FILE *__fp)
{
	TRACE
	(void)__fp;
	return (size_t)0;
}
