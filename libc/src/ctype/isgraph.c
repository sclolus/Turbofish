#include <ltrace.h>
#include <ctype.h>

int isgraph(int c)
{
	TRACE
	return (c >= 0x21 && c <= 0x7e);
}
