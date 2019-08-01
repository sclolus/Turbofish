
#include <string.h>
#include <i386.h>

/*
** XXX This function is not secure ! Only size multiplied by 4 works !
*/

void	aligned_bzero(void *s, size_t n)
{
	uint32_t *dst;

	dst = (uint32_t *)s;
	n >>= 2;
	while (n--)
		*dst++ = (uint32_t)0;
}
