#include <ltrace.h>
#include <ctype.h>

int islower(int c)
{
	TRACE
	return (c >= 'a' && c <= 'z');
}
