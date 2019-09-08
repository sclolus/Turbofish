#include <ltrace.h>
#include <ctype.h>

int isupper(int c)
{
	TRACE
	return (c >= 'A' && c <= 'Z');
}
