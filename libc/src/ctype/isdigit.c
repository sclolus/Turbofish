#include <ltrace.h>
#include <ctype.h>

int isdigit(int c)
{
	TRACE
	return (c >= '0' && c <= '9');
}
