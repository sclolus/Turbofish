
#include "buddy.h"

#include "libft.h"

static u8 physical_buddy[1024 * 512] __attribute__((aligned(32))) = {0};

/*
 * Addressing of all 4go memory
 */
void	*get_physical_addr(u32 page_request)
{
	if (page_request == 0)
		return (void *)MAP_FAILED;

	if (!IS_USABLE(physical_buddy, 1))
		return (void *)MAP_FAILED;

	return (void *)get_mem_area(physical_buddy, page_request, 1, 0);
}

int	drop_physical_addr(void *addr)
{
	return free_mem_area(physical_buddy, (u32)addr, 1, 0);
}

int	mark_physical_area(void *virt_addr, u32 page_request)
{
	return mark_area(physical_buddy, virt_addr, page_request);
}

void	init_physical_map(void *limit_phy_addr)
{
	int ret = mark_area_limit(physical_buddy, (u32)limit_phy_addr, 1, 0);
	if (ret < 0)
		eprintk("%s: unexpected error, limit_phy_addr %p\n",
				__func__, limit_phy_addr);
}
