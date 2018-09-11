
#include "memory_manager.h"
#include "libft.h"

static u32 g_page_mask[21] = {
	0,
	0x1, 0x3, 0x7, 0xF,
	0x1F, 0x3F, 0x7F, 0xFF,
	0x1FF, 0x3FF, 0x7FF, 0xFFF,
	0x1FFF, 0x3FFF, 0x7FFF, 0xFFFF,
	0x1FFFF, 0x3FFFF, 0x7FFFF, 0xFFFFF};

u32	get_mem_area(u32 page_request, u32 index, u32 deep, u8 *map)
{
	u32	ret;

	if (deep == MAX_DEEP || page_request
			> (u32)(GRANULARITY << (MAX_DEEP - deep - 1)))
	{
		if (IS_DIRTY(map, index))
			return MAP_FAILED;
		else
		{
			if (index > (MAP_LENGTH * GRANULARITY_NEG))
				eprintk("%s: ERROR index, got %u\n",
						__func__, index);
			SET(map, index, ALLOCATED);
			u32 segment = (index & g_page_mask[deep])
					* (GRANULARITY << (MAX_DEEP - deep));
			return (((segment >> 10) & 0x3FF) << 22)
					| ((segment & 0x3FF) << 12);
		}
	}

	if (IS_USABLE(map, 2 * index))
	{
		ret = get_mem_area(page_request, 2 * index, deep + 1, map);

		if ((!IS_USABLE(map, 2 * index))
				&& (!IS_USABLE(map, 2 * index + 1)))
		{
			SET(map, index, UNAIVALABLE);
			return ret;
		}

		if (ret != MAP_FAILED) {
			SET(map, index, DIRTY);
			return ret;
		}
	}

	if (IS_USABLE(map, 2 * index + 1))
	{
		ret = get_mem_area(page_request, 2 * index + 1, deep + 1, map);

		if ((!IS_USABLE(map, 2 * index))
				&& (!IS_USABLE(map, 2 * index + 1)))
		{
			SET(map, index, UNAIVALABLE);
			return ret;
		}

		if (ret != MAP_FAILED) {
			SET(map, index, DIRTY);
			return ret;
		}
	}

	return MAP_FAILED;
}

u32	free_mem_area(u32 addr, u32 index, u32 deep, u8 *map)
{
	int ret;

	if (deep > MAX_DEEP)
		return 0;

	u32 ref_addr = (index & g_page_mask[deep])
			* (PAGE_SIZE << (MAX_DEEP - deep)) * GRANULARITY;
	u32 sup_addr = ref_addr + ((PAGE_SIZE << (MAX_DEEP - 1)) >>  deep)
			* GRANULARITY;

	if (addr == ref_addr && IS_ALLOCATED(map, index))
	{
		SET(map, index, UNUSED);
		return GRANULARITY << (MAX_DEEP - deep);
	}
	else if (addr < sup_addr)
		ret = free_mem_area(addr, index * 2, deep + 1, map);
	else
		ret = free_mem_area(addr, index * 2 + 1, deep + 1, map);

	if (ret != 0)
	{
		if (IS_UNUSED(map, index * 2) &&
				IS_UNUSED(map, index * 2 + 1))
			SET(map, index, UNUSED);
	}
	return ret;
}

int	mark_mem_area(u32 addr, u32 index, u32 deep, u32 cap, u8 *map)
{
	int ret;

	if (deep > MAX_DEEP)
		return -1;

	u32 ref_addr = (index & g_page_mask[deep])
			* (PAGE_SIZE << (MAX_DEEP - deep)) * GRANULARITY;
	u32 sup_addr = ref_addr + ((PAGE_SIZE << (MAX_DEEP - 1)) >>  deep)
			* GRANULARITY;

	if (deep == cap)
	{
		if (addr == ref_addr && IS_UNUSED(map, index))
		{
			SET(map, index, ALLOCATED);
			return 0;
		}
		return -1;
	}

	if (addr < sup_addr)
		ret = mark_mem_area(addr, index * 2, deep + 1, cap, map);
	else
		ret = mark_mem_area(addr, index * 2 + 1, deep + 1, cap, map);

	if (ret != -1)
	{
		if ((!IS_USABLE(map, 2 * index))
				&& (!IS_USABLE(map, 2 * index + 1)))
		{
			SET(map, index, UNAIVALABLE);
			return ret;
		}

		SET(map, index, DIRTY);
		return ret;
	}
	return ret;
}
