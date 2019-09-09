#include <ltrace.h>
#include <unistd.h>
#include <limits.h>

int getpagesize(void)
{
	TRACE
	return PAGE_SIZE;
}
