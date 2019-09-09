#include <ltrace.h>
#include <ctype.h>

int iscntrl(int c)
{
	TRACE
	return ((c >= 0 && c <= 31) || c == 127);
}
