
#include "memory_manager.h"

#include "libft.h"

#define PHY_MAP_LOCATION	0xc0780000

static u8 *phy_map;

/*
 * Addressing of all 4go memory
 */
void		*get_physical_addr(u32 page_request)
{
	if (page_request == 0)
		return (void *)MAP_FAILED;

	if (!IS_USABLE(phy_map, 1))
		return (void *)MAP_FAILED;

	return (void *)get_mem_area(phy_map, page_request, 1, 0);
}

int		drop_physical_addr(void *addr)
{
	return free_mem_area(phy_map, (u32)addr, 1, 0);
}

static size_t	count_bits(u32 ref)
{
	size_t count = 0;

	while (ref) {
		count++;
		ref >>= 1;
	}
	return count;
}

int		mark_physical_area(void *addr, u32 page_request)
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
	return mark_mem_area(phy_map, (u32)addr, 1, 0, deep);
}

void		init_physical_map(void *limit_phy_addr)
{
	phy_map = (u8 *)PHY_MAP_LOCATION;
	ft_memset(phy_map, 0, MAP_LENGTH);

	int ret = mark_area_limit(phy_map, (u32)limit_phy_addr, 1, 0);
	if (ret < 0)
		eprintk("%s: unexpected error, limit_phy_addr %p\n",
				__func__, limit_phy_addr);
}
