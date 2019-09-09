#include <ltrace.h>
#include <string.h>

int	toupper(int c)
{
	TRACE
	if (c >= 'a' && c <= 'z')
		return (c - 32);
	return (c);
}
