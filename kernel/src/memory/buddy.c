
#include "memory_manager.h"
#include "libft.h"

static u32 page_mask[21] = {
	0,
	0x1, 0x3, 0x7, 0xF,
	0x1F, 0x3F, 0x7F, 0xFF,
	0x1FF, 0x3FF, 0x7FF, 0xFFF,
	0x1FFF, 0x3FFF, 0x7FFF, 0xFFFF,
	0x1FFFF, 0x3FFFF, 0x7FFFF, 0xFFFFF};

u32	get_mem_area(u8 *map, u32 pages_req, u32 idx, u32 lvl)
{
	u32	ret;

	if (lvl == MAX_LVL || pages_req
			> (u32)(GRANULARITY << (MAX_LVL - lvl - 1)))
	{
		if (IS_DIRTY(map, idx))
			return MAP_FAILED;
		else
		{
			if (idx > (MAP_LENGTH * GRANULARITY_NEG))
				eprintk("%s: ERROR idx, got %u\n",
						__func__, idx);
			SET(map, idx, ALLOCATED);
			u32 segment = (idx & page_mask[lvl])
					* (GRANULARITY << (MAX_LVL - lvl));
			return (((segment >> 10) & 0x3FF) << 22)
					| ((segment & 0x3FF) << 12);
		}
	}

	if (IS_USABLE(map, 2 * idx))
	{
		ret = get_mem_area(map, pages_req, 2 * idx, lvl + 1);

		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1)))
		{
			SET(map, idx, UNAIVALABLE);
			return ret;
		}

		if (ret != MAP_FAILED) {
			SET(map, idx, DIRTY);
			return ret;
		}
	}

	if (IS_USABLE(map, 2 * idx + 1))
	{
		ret = get_mem_area(map, pages_req, 2 * idx + 1, lvl + 1);

		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1)))
		{
			SET(map, idx, UNAIVALABLE);
			return ret;
		}

		if (ret != MAP_FAILED) {
			SET(map, idx, DIRTY);
			return ret;
		}
	}

	return MAP_FAILED;
}

u32	free_mem_area(u8 *map, u32 addr, u32 idx, u32 lvl)
{
	int ret;

	if (lvl > MAX_LVL)
		return 0;

	u32 ref_addr = (idx & page_mask[lvl])
			* (u32)(PAGE_SIZE << (MAX_LVL - lvl)) * GRANULARITY;
	u32 sup_addr = ref_addr + ((u32)(PAGE_SIZE << (MAX_LVL - 1)) >>  lvl)
			* GRANULARITY;

	if (addr == ref_addr && IS_ALLOCATED(map, idx))
	{
		SET(map, idx, UNUSED);
		return GRANULARITY << (MAX_LVL - lvl);
	}
	else if (addr < sup_addr)
		ret = free_mem_area(map, addr, idx * 2, lvl + 1);
	else
		ret = free_mem_area(map, addr, idx * 2 + 1, lvl + 1);

	if (ret != 0)
	{
		if (IS_UNUSED(map, idx * 2) &&
				IS_UNUSED(map, idx * 2 + 1))
			SET(map, idx, UNUSED);
	}
	return ret;
}

int	mark_mem_area(u8 *map, u32 addr, u32 idx, u32 lvl, u32 cap)
{
	int ret;

	if (lvl > MAX_LVL)
		return -1;

	if (!IS_USABLE(map, idx))
		return -1;

	u32 ref_addr = (idx & page_mask[lvl])
			* (u32)(PAGE_SIZE << (MAX_LVL - lvl)) * GRANULARITY;
	u32 sup_addr = ref_addr + ((u32)(PAGE_SIZE << (MAX_LVL - 1)) >> lvl)
			* GRANULARITY;

	if (lvl == cap)
	{
		if (addr == ref_addr && IS_UNUSED(map, idx))
		{
			SET(map, idx, ALLOCATED);
			return lvl;
		}
		return -1;
	}

	if (addr < sup_addr)
		ret = mark_mem_area(map, addr, idx * 2, lvl + 1, cap);
	else
		ret = mark_mem_area(map, addr, idx * 2 + 1, lvl + 1, cap);

	if (ret != -1)
	{
		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1)))
		{
			SET(map, idx, UNAIVALABLE);
			return ret;
		}

		SET(map, idx, DIRTY);
		return ret;
	}
	return ret;
}
