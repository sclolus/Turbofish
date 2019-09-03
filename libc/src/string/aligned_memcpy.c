#include <string.h>
#include <stdint.h>

/*
** XXX This function is not secure ! Only size multiplied by 4 works !
*/

void	*aligned_memcpy(
	void *restrict dst,
	const void *restrict src,
	size_t n)
{
	uint32_t *src1;
	uint32_t *dst1;

	if (src == dst)
		return ((void *)src);
	src1 = (uint32_t *)src;
	dst1 = (uint32_t *)dst;
	n >>= 2;
	while (n--)
		*dst1++ = *src1++;
	return (dst);
}
