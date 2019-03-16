
#include "../memory_manager.h"
#include "buddy.h"

#include "libft.h"

static size_t	count_bits(u32 ref)
{
	size_t count = 0;

	while (ref) {
		count++;
		ref >>= 1;
	}
	return count;
}

int		mark_area(u8 *buddy, void *addr, u32 page_request)
{
	size_t	bitlen;
	u32	deep;

	if (page_request == 0)
		return -1;

	if (page_request <= GRANULARITY) {
		deep = MAX_LVL;
	} else {
		page_request -= 1;
		bitlen = count_bits(page_request);
		// XXX when change granularity, must add a value after 'BITLEN'
		// if granularity = 2, add 1, if granularity = 4, add 2
		deep = MAX_LVL - bitlen + 0;
	}
	return mark_mem_area(buddy, (u32)addr, 1, 0, deep);
}
