#include <ltrace.h>
#include <string.h>

int _tolower(int c)
{
	TRACE
	if (c >= 'A' && c <= 'Z')
		return (c + 32);
	return (c);
}
