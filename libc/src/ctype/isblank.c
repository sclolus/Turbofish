#include <ltrace.h>
#include <ctype.h>

int isblank(int c)
{
	TRACE
	return (c == ' ' || c == '\t');
}
