
#include "memory_manager.h"

#include "libft.h"

static u32 page_mask[21] = {
	0,
	0x1, 0x3, 0x7, 0xF,
	0x1F, 0x3F, 0x7F, 0xFF,
	0x1FF, 0x3FF, 0x7FF, 0xFFF,
	0x1FFF, 0x3FFF, 0x7FFF, 0xFFFF,
	0x1FFFF, 0x3FFFF, 0x7FFFF, 0xFFFFF};

/*
 * According to Buddy algorithm; this function return a free chunk witch
 * can contain the desired number of pages.
 * @initialization: for a search in all the map
 * get_mem_area(map, pages_req, 1, 0);
 * caution, never set 0 as index.
 * @return: Address of pages in the chunk
 * on error: MAP_FAILLED
 */
u32	get_mem_area(u8 *map, u32 pages_req, u32 idx, u32 lvl)
{
	u32 addr;

	if (lvl == MAX_LVL || pages_req
			> (u32)(GRANULARITY << (MAX_LVL - lvl - 1))) {
		if (IS_DIRTY(map, idx)) {
			return MAP_FAILED;
		} else {
			if (idx > (MAP_LENGTH * GRANULARITY_NEG))
				eprintk("%s: ERROR idx, got %u\n",
						__func__, idx);
			SET(map, idx, ALLOCATED);
			u32 segment = (idx & page_mask[lvl])
					* (GRANULARITY << (MAX_LVL - lvl));
			addr = (((segment >> 10) & 0x3FF) << 22)
				| ((segment & 0x3FF) << 12);
			return addr;
		}
	}

	if (IS_USABLE(map, 2 * idx)) {
		addr = get_mem_area(map, pages_req, 2 * idx, lvl + 1);

		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1))) {
			SET(map, idx, UNAIVALABLE);
			return addr;
		}

		if (addr != MAP_FAILED) {
			SET(map, idx, DIRTY);
			return addr;
		}
	}

	if (IS_USABLE(map, 2 * idx + 1)) {
		addr = get_mem_area(map, pages_req, 2 * idx + 1, lvl + 1);

		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1))) {
			SET(map, idx, UNAIVALABLE);
			return addr;
		}

		if (addr != MAP_FAILED) {
			SET(map, idx, DIRTY);
			return addr;
		}
	}

	return MAP_FAILED;
}

/*
 * This function return the container size like 1, 2, 4, 8 or 16 ...
 * The pointed chunk become free
 */
u32	free_mem_area(u8 *map, u32 addr, u32 idx, u32 lvl)
{
	int ret;

	if (lvl > MAX_LVL)
		return 0;

	u32 ref_addr = (idx & page_mask[lvl])
			* (u32)(PAGE_SIZE << (MAX_LVL - lvl)) * GRANULARITY;
	u32 sup_addr = ref_addr + ((u32)(PAGE_SIZE << (MAX_LVL - 1)) >>  lvl)
			* GRANULARITY;

	if (addr == ref_addr && IS_ALLOCATED(map, idx)) {
		SET(map, idx, UNUSED);
		return GRANULARITY << (MAX_LVL - lvl);
	} else if (addr < sup_addr) {
		ret = free_mem_area(map, addr, idx * 2, lvl + 1);
	} else {
		ret = free_mem_area(map, addr, idx * 2 + 1, lvl + 1);
	}

	if (ret != 0) {
		if (IS_UNUSED(map, idx * 2) &&
				IS_UNUSED(map, idx * 2 + 1))
			SET(map, idx, UNUSED);
		else
			SET(map, idx, DIRTY);
	}
	return ret;
}

/*
 * This function make a border between addressable area and not
 * physical memory is not unlimited
 * @parameters
 * 	map: memory map to utilize
 * 	limit_addr: area to make the border
 * 	index: current index on map in recursive scheme
 * 	level: current level on map in recursive scheme
 * @return
 *	integer: 0 on normal, -1 when error occurred
 *
 *	o---o-o			D---o-o    base become dirty
 *	 \   \			 \   \
 *	  \   o                   \   o      free zone
 *	   \         =>            \
 *	    o-o            ------   U-U -- unavailable limit
 *	     \                       \
 *	      o                       o      not allowed zone
 */
int	mark_area_limit(u8 *map, u32 limit_addr, u32 index, u32 level)
{
	u32 ref_addr;
	u32 sup_addr;
	int ret;

	/*
	 * error handling
	 */
	if (limit_addr == 0 || (limit_addr & PAGE_MASK))
		return -1;

	/*
	 * stop condition
	 */
	if (level == MAX_LVL) {
		SET(map, index, UNAIVALABLE);
		return 0;
	}

	/*
	 * recursive descent
	 * ref_addr is the current address location
	 * sup_addr is the upper address of right buddy
	 */
	ref_addr = (index & page_mask[level])
			* (u32)(PAGE_SIZE << (MAX_LVL - level)) * GRANULARITY;
	sup_addr = ref_addr + ((u32)(PAGE_SIZE << (MAX_LVL - 1)) >>  level)
			* GRANULARITY;

	if (limit_addr < sup_addr)
		ret = mark_area_limit(map, limit_addr, index * 2, level + 1);
	else
		ret = mark_area_limit(
				map, limit_addr, index * 2 + 1, level + 1);

	/*
	 * conclusion, while ascent
	 * mark all address equal or greater as unavailable
	 * mark all above address on the way as dirty
	 */
	if (ref_addr == limit_addr)
		SET(map, index, UNAIVALABLE);
	else
		SET(map, index, DIRTY);

	if (sup_addr > limit_addr)
		SET(map, 2 * index + 1, UNAIVALABLE);

	return ret;
}

/*
 * This function directly mark a chunk as ALLOCATED in the desired address
 * on the desired deep (cap). It's necessary to understand exactly how the
 * buddy system work before using this this function.
 * @return: -1 on error, the cap on success
 */
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

	if (lvl == cap) {
		if (addr == ref_addr && IS_UNUSED(map, idx)) {
			SET(map, idx, ALLOCATED);
			return lvl;
		}
		return -1;
	}

	if (addr < sup_addr)
		ret = mark_mem_area(map, addr, idx * 2, lvl + 1, cap);
	else
		ret = mark_mem_area(map, addr, idx * 2 + 1, lvl + 1, cap);

	if (ret != -1) {
		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1))) {
			SET(map, idx, UNAIVALABLE);
			return ret;
		}

		SET(map, idx, DIRTY);
		return ret;
	}
	return ret;
}
