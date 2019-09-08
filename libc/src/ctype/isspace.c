#include <ltrace.h>
#include <ctype.h>

int isspace(int c)
{
	TRACE
	if (c == ' ' ||
	    c == '\f' ||
	    c == '\n' ||
	    c == '\r' ||
	    c == '\t' ||
	    c == '\v') {
		return 1;
	}
	return 0;
}
