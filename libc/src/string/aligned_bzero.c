#include <ltrace.h>
#include <string.h>

/*
** XXX This function is not secure ! Only size multiplied by 4 works !
*/

void	aligned_bzero(void *s, size_t n)
{
	TRACE
	unsigned int *dst;

	dst = (unsigned int *)s;
	n >>= 2;
	while (n--)
		*dst++ = (unsigned int)0;
}
