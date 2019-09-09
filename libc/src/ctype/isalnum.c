#include <ltrace.h>
#include <ctype.h>

int isalnum(int c)
{
	TRACE
	return (isdigit(c) || isalpha(c));
}
